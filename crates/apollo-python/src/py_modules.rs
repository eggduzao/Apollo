use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;

static APOLLO: PyOnceLock<Py<PyModule>> = PyOnceLock::new();
static APOLLO_PLR: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
static UTILS: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
static SERIES: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
static DATAFRAME: PyOnceLock<Py<PyAny>> = PyOnceLock::new();

pub fn apollo(py: Python<'_>) -> &Py<PyModule> {
    APOLLO.get_or_init(py, || py.import("apollo").unwrap().unbind())
}

pub fn apollo_rs(py: Python<'_>) -> &Py<PyAny> {
    APOLLO_PLR.get_or_init(py, || apollo(py).getattr(py, "_plr").unwrap())
}

pub fn pl_utils(py: Python<'_>) -> &Py<PyAny> {
    UTILS.get_or_init(py, || apollo(py).getattr(py, "_utils").unwrap())
}

pub fn pl_series(py: Python<'_>) -> &Py<PyAny> {
    SERIES.get_or_init(py, || apollo(py).getattr(py, "Series").unwrap())
}

pub fn pl_df(py: Python<'_>) -> &Py<PyAny> {
    DATAFRAME.get_or_init(py, || apollo(py).getattr(py, "DataFrame").unwrap())
}
