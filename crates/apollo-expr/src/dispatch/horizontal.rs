use std::borrow::Cow;

use apollo_core::error::{ApolloResult, apollo_bail, apollo_ensure};
use apollo_core::prelude::{Column, DataType, IntoColumn};
use apollo_core::series::Series;
use apollo_core::utils::try_get_supertype;
use apollo_plan::callback::PlanCallback;

pub fn fold(
    c: &[Column],
    callback: &PlanCallback<(Series, Series), Series>,
    returns_scalar: bool,
    return_dtype: Option<&DataType>,
) -> ApolloResult<Column> {
    let mut acc = c[0].clone().take_materialized_series();
    let first_dtype = acc.dtype().clone();
    for c in &c[1..] {
        acc = callback.call((acc.clone(), c.clone().take_materialized_series()))?;
    }
    apollo_ensure!(
        !returns_scalar || acc.len() == 1,
        InvalidOperation: "`fold` is said to return scalar but returned {} elements", acc.len(),
    );
    apollo_ensure!(
        return_dtype.is_none_or(|dt| dt == acc.dtype()),
        ComputeError: "`fold` did not return given return_dtype ({} != {})", return_dtype.unwrap(), acc.dtype()
    );

    if return_dtype.is_none() && acc.dtype() != &first_dtype {
        acc = acc.cast(&first_dtype)?;
    }

    Ok(acc.into_column())
}

pub fn reduce(
    c: &[Column],
    callback: &PlanCallback<(Series, Series), Series>,
    returns_scalar: bool,
    return_dtype: Option<&DataType>,
) -> ApolloResult<Column> {
    let Some(acc) = c.first() else {
        apollo_bail!(ComputeError: "`reduce` did not have any expressions to fold");
    };

    let output_dtype = match return_dtype {
        None => {
            let mut supertype = acc.dtype().clone();
            for c in &c[1..] {
                supertype = try_get_supertype(&supertype, c.dtype())?;
            }
            Cow::Owned(supertype)
        },
        Some(dt) => Cow::Borrowed(dt),
    };
    let output_dtype = output_dtype.as_ref();

    let mut acc = acc.clone().take_materialized_series();
    for c in &c[1..] {
        acc = callback.call((acc.clone(), c.clone().take_materialized_series()))?;
    }

    apollo_ensure!(
        !returns_scalar || acc.len() == 1,
        InvalidOperation: "`reduce` is said to return scalar but returned {} elements", acc.len(),
    );
    apollo_ensure!(
        return_dtype.is_none_or(|dt| dt == acc.dtype()),
        ComputeError: "`reduce` did not return given return_dtype ({} != {})", return_dtype.unwrap(), acc.dtype()
    );

    if acc.dtype() != output_dtype {
        acc = acc.cast(output_dtype)?;
    }

    Ok(acc.into_column())
}

#[cfg(feature = "dtype-struct")]
pub fn cum_reduce(
    c: &[Column],
    callback: &PlanCallback<(Series, Series), Series>,
    returns_scalar: bool,
    return_dtype: Option<&DataType>,
) -> ApolloResult<Column> {
    use apollo_core::prelude::StructChunked;

    let Some(acc) = c.first() else {
        apollo_bail!(ComputeError: "`cum_reduce` did not have any expressions to fold");
    };

    let output_dtype = match return_dtype {
        None => {
            let mut supertype = acc.dtype().clone();
            for c in &c[1..] {
                supertype = try_get_supertype(&supertype, c.dtype())?;
            }
            Cow::Owned(supertype)
        },
        Some(dt) => Cow::Borrowed(dt),
    };
    let output_dtype = output_dtype.as_ref();

    let mut result = Vec::with_capacity(c.len());
    let mut acc = acc.clone().take_materialized_series();
    result.push(acc.clone());
    for c in &c[1..] {
        let name = c.name().clone();
        acc = callback.call((acc.clone(), c.clone().take_materialized_series()))?;

        apollo_ensure!(
            !returns_scalar || acc.len() == 1,
            InvalidOperation: "`cum_reduce` is said to return scalar but returned {} elements", acc.len(),
        );
        apollo_ensure!(
            return_dtype.is_none_or(|dt| dt == acc.dtype()),
            ComputeError: "`cum_reduce` did not return given return_dtype ({} != {})", return_dtype.unwrap(), acc.dtype()
        );

        if acc.dtype() != output_dtype {
            acc = acc.cast(output_dtype)?;
        }

        acc.rename(name);
        result.push(acc.clone());
    }

    StructChunked::from_series(acc.name().clone(), result[0].len(), result.iter())
        .map(|ca| ca.into_column())
}

#[cfg(feature = "dtype-struct")]
pub fn cum_fold(
    c: &[Column],
    callback: &PlanCallback<(Series, Series), Series>,
    returns_scalar: bool,
    return_dtype: Option<&DataType>,
    include_init: bool,
) -> ApolloResult<Column> {
    use apollo_core::prelude::StructChunked;

    let mut result = Vec::with_capacity(c.len());
    let mut acc = c[0].clone().take_materialized_series();

    let output_dtype = return_dtype.map_or_else(|| Cow::Owned(acc.dtype().clone()), Cow::Borrowed);
    let output_dtype = output_dtype.as_ref();

    if include_init {
        result.push(acc.clone())
    }

    for c in &c[1..] {
        let name = c.name().clone();
        acc = callback.call((acc.clone(), c.clone().take_materialized_series()))?;

        apollo_ensure!(
            !returns_scalar || acc.len() == 1,
            InvalidOperation: "`cum_fold` is said to return scalar but returned {} elements", acc.len(),
        );
        apollo_ensure!(
            return_dtype.is_none_or(|dt| dt == acc.dtype()),
            ComputeError: "`cum_fold` did not return given return_dtype ({} != {})", return_dtype.unwrap(), acc.dtype()
        );

        if acc.dtype() != output_dtype {
            acc = acc.cast(output_dtype)?;
        }

        acc.rename(name);
        result.push(acc.clone());
    }

    StructChunked::from_series(acc.name().clone(), result[0].len(), result.iter())
        .map(|ca| ca.into_column())
}
