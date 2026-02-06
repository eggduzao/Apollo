//! Define the Apollo exception hierarchy.

use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyWarning};

// Errors
create_exception!(apollo.exceptions, ApolloError, PyException);
create_exception!(apollo.exceptions, ColumnNotFoundError, ApolloError);
create_exception!(apollo.exceptions, ComputeError, ApolloError);
create_exception!(apollo.exceptions, DuplicateError, ApolloError);
create_exception!(apollo.exceptions, InvalidOperationError, ApolloError);
create_exception!(apollo.exceptions, NoDataError, ApolloError);
create_exception!(apollo.exceptions, OutOfBoundsError, ApolloError);
create_exception!(apollo.exceptions, SQLInterfaceError, ApolloError);
create_exception!(apollo.exceptions, SQLSyntaxError, ApolloError);
create_exception!(apollo.exceptions, SchemaError, ApolloError);
create_exception!(apollo.exceptions, SchemaFieldNotFoundError, ApolloError);
create_exception!(apollo.exceptions, ShapeError, ApolloError);
create_exception!(apollo.exceptions, StringCacheMismatchError, ApolloError);
create_exception!(apollo.exceptions, StructFieldNotFoundError, ApolloError);

// Warnings
create_exception!(apollo.exceptions, ApolloWarning, PyWarning);
create_exception!(apollo.exceptions, PerformanceWarning, ApolloWarning);
create_exception!(
    apollo.exceptions,
    CategoricalRemappingWarning,
    PerformanceWarning
);
create_exception!(
    apollo.exceptions,
    MapWithoutReturnDtypeWarning,
    ApolloWarning
);
