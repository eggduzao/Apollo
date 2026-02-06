use std::sync::Arc;

use apollo_core::error::ApolloResult;
use apollo_core::prelude::*;
use apollo_plan::dsl::{ColumnsUdf, SpecialEq};
use apollo_plan::plans::IRExtensionFunction;

pub fn function_expr_to_udf(func: IRExtensionFunction) -> SpecialEq<Arc<dyn ColumnsUdf>> {
    use IRExtensionFunction::*;
    match func {
        To(dtype) => map!(ext_to, dtype.clone()),
        Storage => map!(ext_storage),
    }
}

fn ext_to(s: &Column, dtype: DataType) -> ApolloResult<Column> {
    let DataType::Extension(typ, storage) = &dtype else {
        apollo_bail!(ComputeError: "ext.to() requires an Extension dtype")
    };

    Ok(s.apply_unary_elementwise(|s| {
        assert!(*s.dtype() == **storage);
        s.clone().into_extension(typ.clone())
    }))
}

fn ext_storage(s: &Column) -> ApolloResult<Column> {
    Ok(s.apply_unary_elementwise(|s| s.to_storage().clone()))
}
