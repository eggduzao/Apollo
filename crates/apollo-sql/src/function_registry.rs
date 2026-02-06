//! This module defines a FunctionRegistry for supported SQL functions and UDFs.

use apollo_error::{ApolloResult, apollo_bail};
pub use apollo_plan::prelude::FunctionOptions;
use apollo_plan::prelude::udf::UserDefinedFunction;
/// A registry that holds user defined functions.
pub trait FunctionRegistry: Send + Sync {
    /// Register a function.
    fn register(&mut self, name: &str, fun: UserDefinedFunction) -> ApolloResult<()>;
    /// Call a user defined function.
    fn get_udf(&self, name: &str) -> ApolloResult<Option<UserDefinedFunction>>;
    /// Check if a function is registered.
    fn contains(&self, name: &str) -> bool;
}

/// A default registry that does not support registering or calling functions.
pub struct DefaultFunctionRegistry {}

impl FunctionRegistry for DefaultFunctionRegistry {
    fn register(&mut self, _name: &str, _fun: UserDefinedFunction) -> ApolloResult<()> {
        apollo_bail!(ComputeError: "'register' not implemented on DefaultFunctionRegistry'")
    }

    fn get_udf(&self, _name: &str) -> ApolloResult<Option<UserDefinedFunction>> {
        apollo_bail!(ComputeError: "'get_udf' not implemented on DefaultFunctionRegistry'")
    }
    fn contains(&self, _name: &str) -> bool {
        false
    }
}
