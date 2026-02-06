use arrow::legacy::is_valid::ArrowArray;

use super::{ObjectArray, ApolloObject};

impl<T: ApolloObject> ArrowArray for ObjectArray<T> {}
