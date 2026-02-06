use std::fmt::{Debug, Formatter};

use apollo::prelude::ApolloError;
use pyo3::create_exception;
use pyo3::exceptions::{
    PyAssertionError, PyException, PyIOError, PyIndexError, PyRuntimeError, PyValueError,
};
use pyo3::prelude::*;
use thiserror::Error;

#[derive(Error)]
pub enum PyApolloErr {
    #[error(transparent)]
    Apollo(#[from] ApolloError),
    #[error("{0}")]
    Other(String),
}

impl std::convert::From<PyApolloErr> for PyErr {
    fn from(err: PyApolloErr) -> PyErr {
        fn convert(err: ApolloError) -> PyErr {
            match err {
                ApolloError::AssertionError(error) => PyAssertionError::new_err(error.to_string()),
                ApolloError::ComputeError(err) => ComputeError::new_err(err.to_string()),
                ApolloError::NoData(err) => NoDataError::new_err(err.to_string()),
                ApolloError::ShapeMismatch(err) => ShapeError::new_err(err.to_string()),
                ApolloError::SchemaMismatch(err) => SchemaError::new_err(err.to_string()),
                ApolloError::IO { error, .. } => PyIOError::new_err(error.to_string()),
                ApolloError::OutOfBounds(err) => PyIndexError::new_err(err.to_string()),
                ApolloError::InvalidOperation(err) => PyValueError::new_err(err.to_string()),
                ApolloError::Duplicate(err) => DuplicateError::new_err(err.to_string()),
                ApolloError::ColumnNotFound(err) => ColumnNotFound::new_err(err.to_string()),
                ApolloError::SchemaFieldNotFound(err) => {
                    SchemaFieldNotFound::new_err(err.to_string())
                },
                ApolloError::StructFieldNotFound(err) => {
                    StructFieldNotFound::new_err(err.to_string())
                },
                ApolloError::StringCacheMismatch(err) => {
                    StringCacheMismatchError::new_err(err.to_string())
                },
                ApolloError::SQLInterface(err) => SQLInterface::new_err(err.to_string()),
                ApolloError::SQLSyntax(err) => SQLSyntax::new_err(err.to_string()),
                ApolloError::Context { error, .. } => convert(*error),
                ApolloError::Python { error } => error.0,
            }
        }

        use PyApolloErr::*;
        match err {
            Apollo(err) => convert(err),
            _ => PyRuntimeError::new_err(format!("{err:?}")),
        }
    }
}

impl Debug for PyApolloErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PyApolloErr::*;
        match self {
            Apollo(err) => write!(f, "{err:?}"),
            Other(err) => write!(f, "BindingsError: {err:?}"),
        }
    }
}

create_exception!(exceptions, AssertionError, PyException);
create_exception!(exceptions, ColumnNotFound, PyException);
create_exception!(exceptions, SchemaFieldNotFound, PyException);
create_exception!(exceptions, StructFieldNotFound, PyException);
create_exception!(exceptions, ComputeError, PyException);
create_exception!(exceptions, NoDataError, PyException);
create_exception!(exceptions, ShapeError, PyException);
create_exception!(exceptions, SchemaError, PyException);
create_exception!(exceptions, DuplicateError, PyException);
create_exception!(exceptions, StringCacheMismatchError, PyException);
create_exception!(exceptions, SQLInterface, PyException);
create_exception!(exceptions, SQLSyntax, PyException);
