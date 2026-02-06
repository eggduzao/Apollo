use super::*;

/// Get the datatypes of function arguments.
///
/// If all arguments give the same datatype or a datatype cannot be found, `Ok(None)` is returned.
pub(super) fn get_function_dtypes(
    input: &[ExprIR],
    expr_arena: &Arena<AExpr>,
    input_schema: &Schema,
    function: &IRFunctionExpr,
) -> ApolloResult<Option<Vec<DataType>>> {
    let mut dtypes = Vec::with_capacity(input.len());
    let mut first = true;
    for e in input {
        let Some((_, dtype)) = get_aexpr_and_type(expr_arena, e.node(), input_schema) else {
            return Ok(None);
        };

        if first {
            check_namespace(function, &dtype)?;
            first = false;
        }
        // Ignore Unknown in the inputs.
        // We will raise if we cannot find the supertype later.
        match dtype {
            DataType::Unknown(UnknownKind::Any) => {
                return Ok(None);
            },
            _ => dtypes.push(dtype),
        }
    }

    if dtypes.iter().all_equal() {
        return Ok(None);
    }
    Ok(Some(dtypes))
}

// `str` namespace belongs to `String`
// `cat` namespace belongs to `Categorical` etc.
fn check_namespace(function: &IRFunctionExpr, first_dtype: &DataType) -> ApolloResult<()> {
    match function {
        #[cfg(feature = "strings")]
        IRFunctionExpr::StringExpr(_) => {
            apollo_ensure!(first_dtype == &DataType::String, InvalidOperation: "expected String type, got: {}", first_dtype)
        },
        IRFunctionExpr::BinaryExpr(_) => {
            apollo_ensure!(first_dtype == &DataType::Binary, InvalidOperation: "expected Binary type, got: {}", first_dtype)
        },
        #[cfg(feature = "temporal")]
        IRFunctionExpr::TemporalExpr(_) => {
            apollo_ensure!(first_dtype.is_temporal(), InvalidOperation: "expected Date(time)/Duration type, got: {}", first_dtype)
        },
        IRFunctionExpr::ListExpr(_) => {
            apollo_ensure!(matches!(first_dtype, DataType::List(_)), InvalidOperation: "expected List type, got: {}", first_dtype)
        },
        #[cfg(feature = "dtype-array")]
        IRFunctionExpr::ArrayExpr(_) => {
            apollo_ensure!(matches!(first_dtype, DataType::Array(_, _)), InvalidOperation: "expected Array type, got: {}", first_dtype)
        },
        #[cfg(feature = "dtype-struct")]
        IRFunctionExpr::StructExpr(_) => {
            apollo_ensure!(matches!(first_dtype, DataType::Struct(_)), InvalidOperation: "expected Struct type, got: {}", first_dtype)
        },
        #[cfg(feature = "dtype-categorical")]
        IRFunctionExpr::Categorical(_) => {
            apollo_ensure!(matches!(first_dtype, DataType::Categorical(_, _)), InvalidOperation: "expected Categorical type, got: {}", first_dtype)
        },
        _ => {},
    }

    Ok(())
}
