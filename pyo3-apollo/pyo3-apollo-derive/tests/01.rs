use apollo_core::error::ApolloResult;
use apollo_core::prelude::*;
use apollo_plan::plans::FieldsMapper;
use pyo3_apollo_derive::apollo_expr;

fn horizontal_product_output(input_fields: &[Field]) -> ApolloResult<Field> {
    FieldsMapper::new(input_fields).map_to_supertype()
}

#[apollo_expr(output_type_func=horizontal_product_output)]
fn horizontal_product(series: &[Series], kwargs: Option<&str>) -> ApolloResult<Series> {
    let _ = kwargs;

    let mut acc = series[0].clone();
    for s in &series[1..] {
        acc = (&acc * s)?
    }
    Ok(acc)
}

fn main() {}
