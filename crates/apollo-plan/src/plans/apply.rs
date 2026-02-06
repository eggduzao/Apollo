use std::fmt::{Debug, Formatter};

use apollo_core::prelude::*;

pub trait DataFrameUdf: Send + Sync {
    fn call_udf(&self, df: DataFrame) -> ApolloResult<DataFrame>;
    fn display_str(&self) -> PlSmallStr {
        PlSmallStr::from_static("dyn DataFrameUdf")
    }
}

impl<F> DataFrameUdf for F
where
    F: Fn(DataFrame) -> ApolloResult<DataFrame> + Send + Sync,
{
    fn call_udf(&self, df: DataFrame) -> ApolloResult<DataFrame> {
        self(df)
    }
}

pub trait DataFrameUdfMut: Send + Sync {
    fn call_udf(&mut self, df: DataFrame) -> ApolloResult<DataFrame>;
}

impl<F> DataFrameUdfMut for F
where
    F: FnMut(DataFrame) -> ApolloResult<DataFrame> + Send + Sync,
{
    fn call_udf(&mut self, df: DataFrame) -> ApolloResult<DataFrame> {
        self(df)
    }
}

impl Debug for dyn DataFrameUdf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.display_str().fmt(f)
    }
}
impl Debug for dyn DataFrameUdfMut {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "dyn DataFrameUdfMut")
    }
}

pub trait UdfSchema: Send + Sync {
    fn get_schema(&self, input_schema: &Schema) -> ApolloResult<SchemaRef>;
}

impl<F> UdfSchema for F
where
    F: Fn(&Schema) -> ApolloResult<SchemaRef> + Send + Sync,
{
    fn get_schema(&self, input_schema: &Schema) -> ApolloResult<SchemaRef> {
        self(input_schema)
    }
}

impl Debug for dyn UdfSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "dyn UdfSchema")
    }
}
