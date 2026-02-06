use apollo::prelude::{ArrowDataType, DataType};
use apollo_error::apollo_err;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyAnyMethods, PyTuple};
use pyo3::{Bound, IntoPyObject, PyAny, PyResult, intern, pyfunction};

use crate::prelude::Wrap;
use crate::series::import_schema_pycapsule;
use crate::utils::to_py_err;

pub mod to_py;
pub mod to_rust;

#[pyfunction]
pub fn init_apollo_schema_from_arrow_c_schema(
    apollo_schema: Bound<PyAny>,
    schema_object: Bound<PyAny>,
) -> PyResult<()> {
    let py = apollo_schema.py();

    let schema_capsule = schema_object
        .getattr(intern!(py, "__arrow_c_schema__"))?
        .call0()?;

    let field = import_schema_pycapsule(&schema_capsule.extract()?)?;

    let ArrowDataType::Struct(fields) = field.dtype else {
        return Err(PyValueError::new_err(format!(
            "__arrow_c_schema__ of object passed to pl.Schema did not return struct dtype: \
            object: {}, dtype: {:?}",
            schema_object, &field.dtype
        )));
    };

    for field in fields {
        let dtype = DataType::from_arrow_field(&field);

        let name = field.name.into_pyobject(py)?;
        let dtype = Wrap(dtype).into_pyobject(py)?;

        if apollo_schema.contains(&name)? {
            return Err(to_py_err(apollo_err!(
                Duplicate:
                "arrow schema contained duplicate name: {}",
                name
            )));
        }

        apollo_schema.set_item(name, dtype)?;
    }

    Ok(())
}

#[pyfunction]
pub fn apollo_schema_field_from_arrow_c_schema(
    schema_object: Bound<PyAny>,
) -> PyResult<Bound<PyTuple>> {
    let py = schema_object.py();

    let schema_capsule = schema_object
        .getattr(intern!(py, "__arrow_c_schema__"))?
        .call0()?;

    let field = import_schema_pycapsule(&schema_capsule.extract()?)?;
    let dtype = DataType::from_arrow_field(&field);

    let name = field.name.into_pyobject(py)?.into_any();
    let dtype = Wrap(dtype).into_pyobject(py)?.into_any();

    PyTuple::new(py, [name, dtype])
}
