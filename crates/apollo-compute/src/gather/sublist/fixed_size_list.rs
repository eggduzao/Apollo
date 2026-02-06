use arrow::array::{ArrayRef, FixedSizeListArray, PrimitiveArray};
use arrow::legacy::prelude::*;
use arrow::legacy::utils::CustomIterTools;
use apollo_error::{ApolloResult, apollo_bail};
use apollo_utils::IdxSize;
use apollo_utils::index::NullCount;

use crate::gather::take_unchecked;

fn sub_fixed_size_list_get_indexes_literal(width: usize, len: usize, index: i64) -> IdxArr {
    (0..len)
        .map(|i| {
            if index >= width as i64 {
                return None;
            }

            index
                .negative_to_usize(width)
                .map(|idx| (idx + i * width) as IdxSize)
        })
        .collect_trusted()
}

fn sub_fixed_size_list_get_indexes(width: usize, index: &PrimitiveArray<i64>) -> IdxArr {
    index
        .iter()
        .enumerate()
        .map(|(i, idx)| {
            if let Some(idx) = idx {
                if *idx >= width as i64 {
                    return None;
                }

                idx.negative_to_usize(width)
                    .map(|idx| (idx + i * width) as IdxSize)
            } else {
                None
            }
        })
        .collect_trusted()
}

pub fn sub_fixed_size_list_get_literal(
    arr: &FixedSizeListArray,
    index: i64,
    null_on_oob: bool,
) -> ApolloResult<ArrayRef> {
    let take_by = sub_fixed_size_list_get_indexes_literal(arr.size(), arr.len(), index);
    if !null_on_oob && take_by.null_count() > 0 {
        apollo_bail!(ComputeError: "get index is out of bounds");
    }

    let values = arr.values();
    // SAFETY:
    // the indices we generate are in bounds
    unsafe { Ok(take_unchecked(&**values, &take_by)) }
}

pub fn sub_fixed_size_list_get(
    arr: &FixedSizeListArray,
    index: &PrimitiveArray<i64>,
    null_on_oob: bool,
) -> ApolloResult<ArrayRef> {
    let take_by = sub_fixed_size_list_get_indexes(arr.size(), index);
    if !null_on_oob && take_by.null_count() > 0 {
        apollo_bail!(ComputeError: "get index is out of bounds");
    }

    let values = arr.values();
    // SAFETY:
    // the indices we generate are in bounds
    unsafe { Ok(take_unchecked(&**values, &take_by)) }
}
