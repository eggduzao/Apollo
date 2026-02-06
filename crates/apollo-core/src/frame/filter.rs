use apollo_error::{ApolloResult, apollo_ensure};

use crate::frame::DataFrame;
use crate::prelude::BooleanChunked;

pub(super) fn filter_zero_width(height: usize, mask: &BooleanChunked) -> ApolloResult<DataFrame> {
    let new_height = if mask.len() == 1 {
        match mask.get(0) {
            Some(true) => height,
            _ => 0,
        }
    } else {
        apollo_ensure!(
            height == mask.len(),
            ShapeMismatch:
            "cannot filter DataFrame of height {} with mask of length {}",
            height, mask.len(),
        );

        mask.num_trues()
    };

    Ok(DataFrame::empty_with_height(new_height))
}
