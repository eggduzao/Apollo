use apollo_core::error::ApolloResult;
use apollo_core::prelude::*;
use pyo3_apollo_derive::apollo_expr;

#[apollo_expr(output_type=Int32)]
fn horizontal_product(series: &[Series], kwargs: Option<&str>) -> ApolloResult<Series> {
    let _ = kwargs;

    let mut acc = series[0].clone();
    for s in &series[1..] {
        acc = (&acc * s)?
    }
    Ok(acc)
}

fn main() {}
