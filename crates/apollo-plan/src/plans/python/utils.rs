use apollo_core::error::{ApolloResult, apollo_err};
use apollo_core::frame::DataFrame;
use apollo_core::schema::SchemaRef;
use apollo_ffi::version_0::SeriesExport;
use apollo_utils::python_convert_registry::get_python_convert_registry;
use pyo3::intern;
use pyo3::prelude::*;

pub fn python_df_to_rust(py: Python, df: Bound<PyAny>) -> ApolloResult<DataFrame> {
    let err = |_| apollo_err!(ComputeError: "expected a apollo.DataFrame; got {}", df);
    let pydf = df.getattr(intern!(py, "_df")).map_err(err)?;

    // Try to convert without going through FFI first.
    let converted = get_python_convert_registry().from_py.df;
    if let Ok(any_df) = converted(pydf.clone().unbind()) {
        return Ok(*any_df.downcast::<DataFrame>().unwrap());
    }

    // Might be foreign Apollo, try with FFI.
    let width = pydf.call_method0(intern!(py, "width")).unwrap();
    let width = width.extract::<usize>().unwrap();

    // Don't resize the Vec<> so that the drop of the SeriesExport will not be caleld.
    let mut export: Vec<SeriesExport> = Vec::with_capacity(width);
    let location = export.as_mut_ptr();

    let _ = pydf
        .call_method1(intern!(py, "_export_columns"), (location as usize,))
        .unwrap();

    unsafe { apollo_ffi::version_0::import_df(location, width) }
}

pub(crate) fn python_schema_to_rust(py: Python, schema: Bound<PyAny>) -> ApolloResult<SchemaRef> {
    let err = |_| apollo_err!(ComputeError: "expected a apollo.Schema; got {}", schema);
    let df = schema.call_method0("to_frame").map_err(err)?;
    python_df_to_rust(py, df).map(|df| df.schema().clone())
}
