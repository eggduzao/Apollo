mod parallel_jaccard_mod;

use apollo::prelude::*;
use apollo_lazy::frame::IntoLazy;
use apollo_lazy::prelude::LazyFrame;
use pyo3::prelude::*;
use pyo3_apollo::error::PyApolloErr;
use pyo3_apollo::{ApolloAllocator, PyDataFrame, PyLazyFrame};

#[global_allocator]
static ALLOC: ApolloAllocator = ApolloAllocator::new();

#[pyfunction]
fn parallel_jaccard(pydf: PyDataFrame, col_a: &str, col_b: &str) -> PyResult<PyDataFrame> {
    let df: DataFrame = pydf.into();
    let df = parallel_jaccard_mod::parallel_jaccard(df, col_a, col_b).map_err(PyApolloErr::from)?;
    Ok(PyDataFrame(df))
}

#[pyfunction]
fn debug(pydf: PyDataFrame) -> PyResult<PyDataFrame> {
    let df: DataFrame = pydf.into();
    eprintln!("{}", &df);
    Ok(PyDataFrame(df))
}

#[pyfunction]
fn empty_df() -> PyResult<PyDataFrame> {
    Ok(PyDataFrame(DataFrame::empty()))
}

#[pyfunction]
fn lazy_parallel_jaccard(pydf: PyLazyFrame, col_a: &str, col_b: &str) -> PyResult<PyLazyFrame> {
    let df: LazyFrame = pydf.into();
    let df = parallel_jaccard_mod::parallel_jaccard(df.collect().unwrap(), col_a, col_b)
        .map_err(PyApolloErr::from)?;
    Ok(PyLazyFrame(df.lazy()))
}

/// A Python module implemented in Rust.
#[pymodule]
fn extend_apollo(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parallel_jaccard, m)?)?;
    m.add_function(wrap_pyfunction!(lazy_parallel_jaccard, m)?)?;
    m.add_function(wrap_pyfunction!(debug, m)?)?;
    m.add_function(wrap_pyfunction!(empty_df, m)?)?;
    Ok(())
}
