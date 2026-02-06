use apollo_core::prelude::ApolloResult;

use crate::prelude::*;

#[cfg(feature = "serde")]
pub fn serialize(expr: &Expr) -> ApolloResult<Option<Vec<u8>>> {
    let mut buf = vec![];
    apollo_utils::pl_serialize::serialize_into_writer::<_, _, true>(&mut buf, expr)?;

    Ok(Some(buf))
}
