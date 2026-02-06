use std::sync::Arc;

use either::Either;
use apollo_core::error::{ApolloResult, apollo_err};
use apollo_core::schema::SchemaRef;
use apollo_utils::python_function::PythonFunction;
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::dsl::SpecialEq;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "dsl-schema", derive(schemars::JsonSchema))]
pub struct PythonOptionsDsl {
    /// A function that returns a Python Generator.
    /// The generator should produce Apollo DataFrame's.
    pub scan_fn: Option<PythonFunction>,
    /// Either the schema fn or schema is set.
    pub schema_fn: Option<SpecialEq<Arc<Either<PythonFunction, SchemaRef>>>>,
    pub python_source: PythonScanSource,
    pub validate_schema: bool,
    pub is_pure: bool,
}

impl PythonOptionsDsl {
    pub fn get_schema(&self) -> ApolloResult<SchemaRef> {
        match self.schema_fn.as_ref().expect("should be set").as_ref() {
            Either::Left(func) => Python::attach(|py| {
                let schema = func
                    .0
                    .call0(py)
                    .map_err(|e| apollo_err!(ComputeError: "schema callable failed: {}", e))?;
                crate::plans::python::python_schema_to_rust(py, schema.into_bound(py))
            }),
            Either::Right(schema) => Ok(schema.clone()),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "dsl-schema", derive(schemars::JsonSchema))]
pub enum PythonScanSource {
    Pyarrow,
    Cuda,
    #[default]
    IOPlugin,
}
