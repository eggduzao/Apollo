//! # Data types supported by Apollo.
//!
//! At the moment Apollo doesn't include all data types available by Arrow. The goal is to
//! incrementally support more data types and prioritize these by usability.
//!
//! [See the AnyValue variants](enum.AnyValue.html#variants) for the data types that
//! are currently supported.
//!
#[cfg(feature = "serde")]
mod _serde;
mod aliases;
mod any_value;
mod dtype;
#[cfg(feature = "dtype-extension")]
pub mod extension;
mod field;
mod into_scalar;
#[cfg(feature = "object")]
mod static_array_collect;
mod temporal;

#[cfg(feature = "proptest")]
pub mod proptest;

use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign};
use std::sync::Arc;

mod schema;
pub use aliases::*;
pub use any_value::*;
pub use arrow::array::{ArrayCollectIterExt, ArrayFromIter, ArrayFromIterDtype, StaticArray};
#[cfg(feature = "dtype-categorical")]
use arrow::datatypes::IntegerType;
pub use arrow::datatypes::reshape::*;
pub use arrow::datatypes::{ArrowDataType, TimeUnit as ArrowTimeUnit};
use arrow::types::NativeType;
use bytemuck::Zeroable;
pub use dtype::*;
pub use field::*;
pub use into_scalar::*;
use num_traits::{AsPrimitive, Bounded, FromPrimitive, Num, NumCast, One, Zero};
use apollo_compute::arithmetic::HasPrimitiveArithmeticKernel;
use apollo_compute::float_sum::FloatSum;
#[cfg(feature = "dtype-categorical")]
pub use apollo_dtype::categorical::{
    CatNative, CatSize, CategoricalMapping, CategoricalPhysical, Categories, FrozenCategories,
    ensure_same_categories, ensure_same_frozen_categories,
};
use apollo_utils::abs_diff::AbsDiff;
use apollo_utils::float::IsFloat;
use apollo_utils::float16::pf16;
use apollo_utils::min_max::MinMax;
use apollo_utils::nulls::IsNull;
use apollo_utils::total_ord::TotalHash;
pub use schema::SchemaExtPl;
#[cfg(any(feature = "serde", feature = "serde-lazy"))]
use serde::{Deserialize, Serialize};
#[cfg(any(feature = "serde", feature = "serde-lazy"))]
use serde::{Deserializer, Serializer};
pub use temporal::*;

pub use crate::chunked_array::logical::*;
#[cfg(feature = "object")]
use crate::chunked_array::object::ObjectArray;
#[cfg(feature = "object")]
use crate::chunked_array::object::ApolloObjectSafe;
use crate::prelude::*;
use crate::series::implementations::SeriesWrap;
use crate::utils::Wrap;

pub struct TrueT;
pub struct FalseT;

/// # Safety
///
/// The StaticArray and dtype return must be correct.
pub unsafe trait ApolloDataType: Send + Sync + Sized + 'static {
    type Physical<'a>: std::fmt::Debug + Clone;
    type OwnedPhysical: std::fmt::Debug + Send + Sync + Clone + PartialEq;
    type ZeroablePhysical<'a>: Zeroable + From<Self::Physical<'a>>;
    type Array: for<'a> StaticArray<
            ValueT<'a> = Self::Physical<'a>,
            ZeroableValueT<'a> = Self::ZeroablePhysical<'a>,
        >;
    type IsNested;
    type HasViews;
    type IsStruct;
    type IsObject;

    /// Returns the DataType variant associated with this ApolloDataType.
    /// Not implemented for types whose DataTypes have parameters.
    fn get_static_dtype() -> DataType
    where
        Self: Sized;
}

pub trait ApolloPhysicalType: ApolloDataType {
    // A physical type is one backed by a ChunkedArray directly, as opposed to
    // logical types which wrap physical data.
    fn ca_into_series(ca: ChunkedArray<Self>) -> Series;
}

pub trait ApolloNumericType: ApolloPhysicalType + 'static
where
    Self: for<'a> ApolloDataType<
            OwnedPhysical = Self::Native,
            Physical<'a> = Self::Native,
            ZeroablePhysical<'a> = Self::Native,
            Array = PrimitiveArray<Self::Native>,
            IsNested = FalseT,
            HasViews = FalseT,
            IsStruct = FalseT,
            IsObject = FalseT,
        >,
{
    type Native: NumericNative;
}

pub trait ApolloIntegerType: ApolloNumericType {}
pub trait ApolloFloatType: ApolloNumericType {}

/// # Safety
/// The physical() return type must be correct for Native.
#[cfg(feature = "dtype-categorical")]
pub unsafe trait ApolloCategoricalType: ApolloDataType {
    type Native: NumericNative + CatNative + DictionaryKey + PartialEq + Eq + Hash;
    type ApolloPhysical: ApolloIntegerType<Native = Self::Native>;

    fn physical() -> CategoricalPhysical;
}

macro_rules! impl_apollo_num_datatype {
    ($trait: ident, $pdt:ident, $variant:ident, $physical:ty, $owned_phys:ty) => {
        #[derive(Clone, Copy)]
        pub struct $pdt {}

        unsafe impl ApolloDataType for $pdt {
            type Physical<'a> = $physical;
            type OwnedPhysical = $owned_phys;
            type ZeroablePhysical<'a> = $physical;
            type Array = PrimitiveArray<$physical>;
            type IsNested = FalseT;
            type HasViews = FalseT;
            type IsStruct = FalseT;
            type IsObject = FalseT;

            #[inline]
            fn get_static_dtype() -> DataType {
                DataType::$variant
            }
        }

        impl ApolloNumericType for $pdt {
            type Native = $physical;
        }

        impl $trait for $pdt {}
    };
}

macro_rules! impl_apollo_datatype {
    ($pdt:ident, $dtype:expr, $arr:ty, $lt:lifetime, $phys:ty, $zerophys:ty, $owned_phys:ty, $has_views:ident) => {
        #[derive(Clone, Copy)]
        pub struct $pdt {}

        unsafe impl ApolloDataType for $pdt {
            type Physical<$lt> = $phys;
            type OwnedPhysical = $owned_phys;
            type ZeroablePhysical<$lt> = $zerophys;
            type Array = $arr;
            type IsNested = FalseT;
            type HasViews = $has_views;
            type IsStruct = FalseT;
            type IsObject = FalseT;

            #[inline]
            fn get_static_dtype() -> DataType {
                $dtype
            }
        }
    };
}

macro_rules! impl_apollo_categorical_datatype {
    ($pdt:ident, $phys:ty, $native:ty, $phys_variant:ident) => {
        impl_apollo_datatype!(
            $pdt,
            unimplemented!(),
            PrimitiveArray<$native>,
            'a,
            $native,
            $native,
            $native,
            FalseT
        );

        #[cfg(feature = "dtype-categorical")]
        unsafe impl ApolloCategoricalType for $pdt {
            type Native = $native;
            type ApolloPhysical = $phys;

            fn physical() -> CategoricalPhysical {
                CategoricalPhysical::$phys_variant
            }
        }
    }
}

impl_apollo_num_datatype!(ApolloIntegerType, UInt8Type, UInt8, u8, u8);
impl_apollo_num_datatype!(ApolloIntegerType, UInt16Type, UInt16, u16, u16);
impl_apollo_num_datatype!(ApolloIntegerType, UInt32Type, UInt32, u32, u32);
impl_apollo_num_datatype!(ApolloIntegerType, UInt64Type, UInt64, u64, u64);
#[cfg(feature = "dtype-u128")]
impl_apollo_num_datatype!(ApolloIntegerType, UInt128Type, UInt128, u128, u128);
impl_apollo_num_datatype!(ApolloIntegerType, Int8Type, Int8, i8, i8);
impl_apollo_num_datatype!(ApolloIntegerType, Int16Type, Int16, i16, i16);
impl_apollo_num_datatype!(ApolloIntegerType, Int32Type, Int32, i32, i32);
impl_apollo_num_datatype!(ApolloIntegerType, Int64Type, Int64, i64, i64);
#[cfg(feature = "dtype-i128")]
impl_apollo_num_datatype!(ApolloIntegerType, Int128Type, Int128, i128, i128);
#[cfg(feature = "dtype-f16")]
impl_apollo_num_datatype!(ApolloFloatType, Float16Type, Float16, pf16, pf16);
impl_apollo_num_datatype!(ApolloFloatType, Float32Type, Float32, f32, f32);
impl_apollo_num_datatype!(ApolloFloatType, Float64Type, Float64, f64, f64);

impl_apollo_datatype!(StringType, DataType::String, Utf8ViewArray, 'a, &'a str, Option<&'a str>, String, TrueT);
impl_apollo_datatype!(BinaryType, DataType::Binary, BinaryViewArray, 'a, &'a [u8], Option<&'a [u8]>, Box<[u8]>, TrueT);
impl_apollo_datatype!(BinaryOffsetType, DataType::BinaryOffset, BinaryArray<i64>, 'a, &'a [u8], Option<&'a [u8]>, Box<[u8]>, FalseT);
impl_apollo_datatype!(BooleanType, DataType::Boolean, BooleanArray, 'a, bool, bool, bool, FalseT);

#[cfg(feature = "dtype-decimal")]
impl_apollo_datatype!(DecimalType, unimplemented!(), PrimitiveArray<i128>, 'a, i128, i128, i128, FalseT);
impl_apollo_datatype!(DatetimeType, unimplemented!(), PrimitiveArray<i64>, 'a, i64, i64, i64, FalseT);
impl_apollo_datatype!(DurationType, unimplemented!(), PrimitiveArray<i64>, 'a, i64, i64, i64, FalseT);
impl_apollo_datatype!(CategoricalType, unimplemented!(), PrimitiveArray<u32>, 'a, u32, u32, u32, FalseT);
impl_apollo_datatype!(DateType, DataType::Date, PrimitiveArray<i32>, 'a, i32, i32, i32, FalseT);
impl_apollo_datatype!(TimeType, DataType::Time, PrimitiveArray<i64>, 'a, i64, i64, i64, FalseT);

impl_apollo_categorical_datatype!(Categorical8Type, UInt8Type, u8, U8);
impl_apollo_categorical_datatype!(Categorical16Type, UInt16Type, u16, U16);
impl_apollo_categorical_datatype!(Categorical32Type, UInt32Type, u32, U32);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ListType {}
unsafe impl ApolloDataType for ListType {
    type Physical<'a> = Box<dyn Array>;
    type OwnedPhysical = Box<dyn Array>;
    type ZeroablePhysical<'a> = Option<Box<dyn Array>>;
    type Array = ListArray<i64>;
    type IsNested = TrueT;
    type HasViews = FalseT;
    type IsStruct = FalseT;
    type IsObject = FalseT;

    fn get_static_dtype() -> DataType {
        // Null as we cannot know anything without self.
        DataType::List(Box::new(DataType::Null))
    }
}

impl ApolloPhysicalType for ListType {
    fn ca_into_series(ca: ChunkedArray<Self>) -> Series {
        Series(Arc::new(SeriesWrap(ca)))
    }
}

#[cfg(feature = "dtype-struct")]
pub struct StructType {}
#[cfg(feature = "dtype-struct")]
unsafe impl ApolloDataType for StructType {
    // The physical types are invalid.
    // We don't want these to be used as that would be
    // very expensive. We use const asserts to ensure
    // traits/methods using the physical types are
    // not called for structs.
    type Physical<'a> = ();
    type OwnedPhysical = ();
    type ZeroablePhysical<'a> = ();
    type Array = StructArray;
    type IsNested = TrueT;
    type HasViews = FalseT;
    type IsStruct = TrueT;
    type IsObject = FalseT;

    fn get_static_dtype() -> DataType
    where
        Self: Sized,
    {
        DataType::Struct(vec![])
    }
}

#[cfg(feature = "dtype-array")]
pub struct FixedSizeListType {}
#[cfg(feature = "dtype-array")]
unsafe impl ApolloDataType for FixedSizeListType {
    type Physical<'a> = Box<dyn Array>;
    type OwnedPhysical = Box<dyn Array>;
    type ZeroablePhysical<'a> = Option<Box<dyn Array>>;
    type Array = FixedSizeListArray;
    type IsNested = TrueT;
    type HasViews = FalseT;
    type IsStruct = FalseT;
    type IsObject = FalseT;

    fn get_static_dtype() -> DataType {
        // Null as we cannot know anything without self.
        DataType::Array(Box::new(DataType::Null), 0)
    }
}

#[cfg(feature = "object")]
pub struct ObjectType<T>(T);
#[cfg(feature = "object")]
unsafe impl<T: ApolloObject> ApolloDataType for ObjectType<T> {
    type Physical<'a> = &'a T;
    type OwnedPhysical = T;
    type ZeroablePhysical<'a> = Option<&'a T>;
    type Array = ObjectArray<T>;
    type IsNested = FalseT;
    type HasViews = FalseT;
    type IsStruct = FalseT;
    type IsObject = TrueT;

    fn get_static_dtype() -> DataType {
        DataType::Object(T::type_name())
    }
}

macro_rules! impl_phys_dtype {
    ($pdt:ty) => {
        impl ApolloPhysicalType for $pdt {
            fn ca_into_series(ca: ChunkedArray<Self>) -> Series {
                Series(Arc::new(SeriesWrap(ca)))
            }
        }
    };
}

macro_rules! impl_cond_phys_dtype {
    ($pdt:ty, $feat:literal) => {
        impl ApolloPhysicalType for $pdt {
            fn ca_into_series(ca: ChunkedArray<Self>) -> Series {
                #[cfg(feature = $feat)]
                {
                    Series(Arc::new(SeriesWrap(ca)))
                }

                #[cfg(not(feature = $feat))]
                {
                    unimplemented!()
                }
            }
        }
    };
}

// Annoyingly these types always exist but may not have an implementation to refer to.
impl_cond_phys_dtype!(UInt8Type, "dtype-u8");
impl_cond_phys_dtype!(UInt16Type, "dtype-u16");
impl_cond_phys_dtype!(Int8Type, "dtype-i8");
impl_cond_phys_dtype!(Int16Type, "dtype-i16");

impl_phys_dtype!(Int32Type);
impl_phys_dtype!(Int64Type);
impl_phys_dtype!(UInt32Type);
impl_phys_dtype!(UInt64Type);
#[cfg(feature = "dtype-f16")]
impl_phys_dtype!(Float16Type);
impl_phys_dtype!(Float32Type);
impl_phys_dtype!(Float64Type);

impl_phys_dtype!(StringType);
impl_phys_dtype!(BinaryType);
impl_phys_dtype!(BinaryOffsetType);
impl_phys_dtype!(BooleanType);

#[cfg(feature = "dtype-u128")]
impl_phys_dtype!(UInt128Type);
#[cfg(feature = "dtype-i128")]
impl_phys_dtype!(Int128Type);

#[cfg(feature = "dtype-array")]
impl_phys_dtype!(FixedSizeListType);

#[cfg(feature = "dtype-struct")]
impl_phys_dtype!(StructType);

#[cfg(feature = "object")]
impl<T: ApolloObject> ApolloPhysicalType for ObjectType<T> {
    fn ca_into_series(ca: ChunkedArray<Self>) -> Series {
        Series(Arc::new(SeriesWrap(ca)))
    }
}

#[cfg(feature = "dtype-array")]
pub type ArrayChunked = ChunkedArray<FixedSizeListType>;
pub type ListChunked = ChunkedArray<ListType>;
pub type BooleanChunked = ChunkedArray<BooleanType>;
pub type UInt8Chunked = ChunkedArray<UInt8Type>;
pub type UInt16Chunked = ChunkedArray<UInt16Type>;
pub type UInt32Chunked = ChunkedArray<UInt32Type>;
pub type UInt64Chunked = ChunkedArray<UInt64Type>;
#[cfg(feature = "dtype-u128")]
pub type UInt128Chunked = ChunkedArray<UInt128Type>;
pub type Int8Chunked = ChunkedArray<Int8Type>;
pub type Int16Chunked = ChunkedArray<Int16Type>;
pub type Int32Chunked = ChunkedArray<Int32Type>;
pub type Int64Chunked = ChunkedArray<Int64Type>;
#[cfg(feature = "dtype-i128")]
pub type Int128Chunked = ChunkedArray<Int128Type>;
#[cfg(feature = "dtype-f16")]
pub type Float16Chunked = ChunkedArray<Float16Type>;
pub type Float32Chunked = ChunkedArray<Float32Type>;
pub type Float64Chunked = ChunkedArray<Float64Type>;
pub type StringChunked = ChunkedArray<StringType>;
pub type BinaryChunked = ChunkedArray<BinaryType>;
pub type BinaryOffsetChunked = ChunkedArray<BinaryOffsetType>;
#[cfg(feature = "object")]
pub type ObjectChunked<T> = ChunkedArray<ObjectType<T>>;

pub trait NumericNative:
    TotalOrd
    + PartialOrd
    + TotalHash
    + NativeType
    + Num
    + NumCast
    + Zero
    + One
    // + Simd
    // + Simd8
    + std::iter::Sum<Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + AbsDiff
    + Bounded
    + FromPrimitive
    + IsFloat
    + HasPrimitiveArithmeticKernel<TrueDivT=<Self::TrueDivApolloType as ApolloNumericType>::Native>
    + FloatSum<f64>
    + AsPrimitive<f64>
    + MinMax
    + IsNull
{
    type ApolloType: ApolloNumericType;
    type TrueDivApolloType: ApolloNumericType;
}

impl NumericNative for i8 {
    type ApolloType = Int8Type;
    type TrueDivApolloType = Float64Type;
}
impl NumericNative for i16 {
    type ApolloType = Int16Type;
    type TrueDivApolloType = Float64Type;
}
impl NumericNative for i32 {
    type ApolloType = Int32Type;
    type TrueDivApolloType = Float64Type;
}
impl NumericNative for i64 {
    type ApolloType = Int64Type;
    type TrueDivApolloType = Float64Type;
}
#[cfg(feature = "dtype-i128")]
impl NumericNative for i128 {
    type ApolloType = Int128Type;
    type TrueDivApolloType = Float64Type;
}
impl NumericNative for u8 {
    type ApolloType = UInt8Type;
    type TrueDivApolloType = Float64Type;
}
impl NumericNative for u16 {
    type ApolloType = UInt16Type;
    type TrueDivApolloType = Float64Type;
}
impl NumericNative for u32 {
    type ApolloType = UInt32Type;
    type TrueDivApolloType = Float64Type;
}
impl NumericNative for u64 {
    type ApolloType = UInt64Type;
    type TrueDivApolloType = Float64Type;
}
#[cfg(feature = "dtype-u128")]
impl NumericNative for u128 {
    type ApolloType = UInt128Type;
    type TrueDivApolloType = Float64Type;
}
#[cfg(feature = "dtype-f16")]
impl NumericNative for pf16 {
    type ApolloType = Float16Type;
    type TrueDivApolloType = Float16Type;
}
impl NumericNative for f32 {
    type ApolloType = Float32Type;
    type TrueDivApolloType = Float32Type;
}
impl NumericNative for f64 {
    type ApolloType = Float64Type;
    type TrueDivApolloType = Float64Type;
}
