use pyo3::prelude::*;

#[pyfunction]
pub fn escape_regex(s: &str) -> PyResult<String> {
    let escaped_s = apollo_ops::chunked_array::strings::escape_regex_str(s);
    Ok(escaped_s)
}
