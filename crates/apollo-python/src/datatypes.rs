use apollo::prelude::*;
use apollo_lazy::dsl;
use pyo3::prelude::*;

use crate::error::PyApolloErr;
use crate::{PyExpr, Wrap};

#[pyfunction]
pub fn _get_dtype_max(dt: Wrap<DataType>) -> PyResult<PyExpr> {
    let v = dt.0.max().map_err(PyApolloErr::from)?;
    Ok(dsl::lit(v).into())
}

#[pyfunction]
pub fn _get_dtype_min(dt: Wrap<DataType>) -> PyResult<PyExpr> {
    let v = dt.0.min().map_err(PyApolloErr::from)?;
    Ok(dsl::lit(v).into())
}

#[pyfunction]
pub fn _known_timezones() -> PyResult<Vec<String>> {
    use apollo_time::prelude::known_timezones;
    Ok(known_timezones()
        .iter()
        .map(|tz| tz.to_string())
        .collect::<Vec<_>>())
}
