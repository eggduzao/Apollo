use std::collections::VecDeque;

use apollo_error::{ApolloResult, apollo_err};

use super::super::Node;
use crate::array::NullArray;
use crate::datatypes::ArrowDataType;
use crate::io::ipc::read::array::{try_get_array_length, try_get_field_node};

pub fn read_null(
    field_nodes: &mut VecDeque<Node>,
    dtype: ArrowDataType,
    limit: Option<usize>,
) -> ApolloResult<NullArray> {
    let field_node = try_get_field_node(field_nodes, &dtype)?;

    let length = try_get_array_length(field_node, limit)?;

    NullArray::try_new(dtype, length)
}

pub fn skip_null(field_nodes: &mut VecDeque<Node>) -> ApolloResult<()> {
    let _ = field_nodes.pop_front().ok_or_else(|| {
        apollo_err!(
            oos = "IPC: unable to fetch the field for null. The file or stream is corrupted."
        )
    })?;
    Ok(())
}
