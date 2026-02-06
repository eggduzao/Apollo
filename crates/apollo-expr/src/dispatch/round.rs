use apollo_core::error::ApolloResult;
use apollo_core::prelude::Column;
use apollo_core::series::Series;
use apollo_ops::series::RoundSeries;
use apollo_ops::series::round::RoundMode;

pub(super) fn round(c: &Column, decimals: u32, mode: RoundMode) -> ApolloResult<Column> {
    c.try_apply_unary_elementwise(|s| s.round(decimals, mode))
}

pub(super) fn round_sig_figs(c: &Column, digits: i32) -> ApolloResult<Column> {
    c.try_apply_unary_elementwise(|s| s.round_sig_figs(digits))
}

pub(super) fn floor(c: &Column) -> ApolloResult<Column> {
    c.try_apply_unary_elementwise(Series::floor)
}

pub(super) fn ceil(c: &Column) -> ApolloResult<Column> {
    c.try_apply_unary_elementwise(Series::ceil)
}
