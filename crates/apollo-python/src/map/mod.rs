pub mod lazy;
pub mod series;

use apollo::prelude::*;
pub trait PyApolloNumericType: ApolloNumericType {}

impl PyApolloNumericType for UInt8Type {}
impl PyApolloNumericType for UInt16Type {}
impl PyApolloNumericType for UInt32Type {}
impl PyApolloNumericType for UInt64Type {}
impl PyApolloNumericType for UInt128Type {}
impl PyApolloNumericType for Int8Type {}
impl PyApolloNumericType for Int16Type {}
impl PyApolloNumericType for Int32Type {}
impl PyApolloNumericType for Int64Type {}
impl PyApolloNumericType for Int128Type {}
impl PyApolloNumericType for Float16Type {}
impl PyApolloNumericType for Float32Type {}
impl PyApolloNumericType for Float64Type {}
