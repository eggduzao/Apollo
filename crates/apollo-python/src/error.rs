use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::ErrorKind;

use apollo::prelude::ApolloError;
use apollo_error::ApolloWarning;
use pyo3::PyTypeInfo;
use pyo3::exceptions::{
    PyDeprecationWarning, PyFileExistsError, PyFileNotFoundError, PyIOError, PyPermissionError,
    PyRuntimeError, PyUserWarning,
};
use pyo3::prelude::*;

use crate::Wrap;
use crate::exceptions::{
    CategoricalRemappingWarning, ColumnNotFoundError, ComputeError, DuplicateError,
    InvalidOperationError, MapWithoutReturnDtypeWarning, NoDataError, OutOfBoundsError,
    SQLInterfaceError, SQLSyntaxError, SchemaError, SchemaFieldNotFoundError, ShapeError,
    StringCacheMismatchError, StructFieldNotFoundError,
};

pub enum PyApolloErr {
    Apollo(ApolloError),
    Python(PyErr),
    Other(String),
}

impl Error for PyApolloErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Apollo(err) => Some(err),
            Self::Python(err) => Some(err),
            Self::Other(_) => None,
        }
    }
}

impl std::fmt::Display for PyApolloErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Apollo(err) => Display::fmt(err, f),
            Self::Python(err) => Display::fmt(err, f),
            Self::Other(err) => write!(f, "{err}"),
        }
    }
}

impl From<ApolloError> for PyApolloErr {
    fn from(err: ApolloError) -> Self {
        PyApolloErr::Apollo(err)
    }
}

impl From<PyErr> for PyApolloErr {
    fn from(err: PyErr) -> Self {
        PyApolloErr::Python(err)
    }
}

impl From<PyApolloErr> for PyErr {
    fn from(err: PyApolloErr) -> PyErr {
        use PyApolloErr::*;
        match err {
            Apollo(err) => match err {
                ApolloError::AssertionError(err) => {
                    pyo3::exceptions::PyAssertionError::new_err(err.to_string())
                },
                ApolloError::ColumnNotFound(name) => ColumnNotFoundError::new_err(name.to_string()),
                ApolloError::ComputeError(err) => ComputeError::new_err(err.to_string()),
                ApolloError::Duplicate(err) => DuplicateError::new_err(err.to_string()),
                ApolloError::InvalidOperation(err) => {
                    InvalidOperationError::new_err(err.to_string())
                },
                ApolloError::IO { error, msg } => {
                    let msg = if let Some(msg) = msg {
                        msg.to_string()
                    } else {
                        error.to_string()
                    };
                    match error.kind() {
                        ErrorKind::NotFound => PyFileNotFoundError::new_err(msg),
                        ErrorKind::PermissionDenied => PyPermissionError::new_err(msg),
                        ErrorKind::AlreadyExists => PyFileExistsError::new_err(msg),
                        _ => PyIOError::new_err(msg),
                    }
                },
                ApolloError::NoData(err) => NoDataError::new_err(err.to_string()),
                ApolloError::OutOfBounds(err) => OutOfBoundsError::new_err(err.to_string()),
                ApolloError::SQLInterface(name) => SQLInterfaceError::new_err(name.to_string()),
                ApolloError::SQLSyntax(name) => SQLSyntaxError::new_err(name.to_string()),
                ApolloError::SchemaFieldNotFound(name) => {
                    SchemaFieldNotFoundError::new_err(name.to_string())
                },
                ApolloError::SchemaMismatch(err) => SchemaError::new_err(err.to_string()),
                ApolloError::ShapeMismatch(err) => ShapeError::new_err(err.to_string()),
                ApolloError::StringCacheMismatch(err) => {
                    StringCacheMismatchError::new_err(err.to_string())
                },
                ApolloError::StructFieldNotFound(name) => {
                    StructFieldNotFoundError::new_err(name.to_string())
                },
                ApolloError::Context { .. } => {
                    let tmp = PyApolloErr::Apollo(err.context_trace());
                    PyErr::from(tmp)
                },
                ApolloError::Python { error } => error.0,
            },
            Python(err) => err,
            err => PyRuntimeError::new_err(format!("{:?}", &err)),
        }
    }
}

impl Debug for PyApolloErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PyApolloErr::*;
        match self {
            Apollo(err) => write!(f, "{err:?}"),
            Python(err) => write!(f, "{err:?}"),
            Other(err) => write!(f, "BindingsError: {err:?}"),
        }
    }
}

#[macro_export]
macro_rules! raise_err(
    ($msg:expr, $err:ident) => {{
        Err(ApolloError::$err($msg.into())).map_err(PyApolloErr::from)?;
        unreachable!()
    }}
);

impl<'py> IntoPyObject<'py> for Wrap<ApolloWarning> {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0 {
            ApolloWarning::CategoricalRemappingWarning => {
                Ok(CategoricalRemappingWarning::type_object(py).into_any())
            },
            ApolloWarning::MapWithoutReturnDtypeWarning => {
                Ok(MapWithoutReturnDtypeWarning::type_object(py).into_any())
            },
            ApolloWarning::UserWarning => Ok(PyUserWarning::type_object(py).into_any()),
            ApolloWarning::Deprecation => Ok(PyDeprecationWarning::type_object(py).into_any()),
        }
    }
}
