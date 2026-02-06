use std::ops::Not;

use apollo_core::with_match_physical_integer_apollo_type;

use super::*;

pub fn negate_bitwise(s: &Series) -> ApolloResult<Series> {
    match s.dtype() {
        DataType::Boolean => Ok(s.bool().unwrap().not().into_series()),
        dt if dt.is_integer() => {
            with_match_physical_integer_apollo_type!(dt, |$T| {
                let ca: &ChunkedArray<$T> = s.as_any().downcast_ref().unwrap();
                Ok(ca.apply_values(|v| !v).into_series())
            })
        },
        dt => apollo_bail!(InvalidOperation: "dtype {:?} not supported in 'not' operation", dt),
    }
}
