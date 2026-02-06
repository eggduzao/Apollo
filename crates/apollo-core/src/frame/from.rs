use crate::prelude::*;

impl TryFrom<StructArray> for DataFrame {
    type Error = ApolloError;

    fn try_from(arr: StructArray) -> ApolloResult<Self> {
        let (fld, height, arrs, nulls) = arr.into_data();
        apollo_ensure!(
            nulls.is_none(),
            ComputeError: "cannot deserialize struct with nulls into a DataFrame"
        );
        let columns = fld
            .iter()
            .zip(arrs)
            .map(|(fld, arr)| {
                // SAFETY:
                // reported data type is correct
                unsafe {
                    Series::_try_from_arrow_unchecked_with_md(
                        fld.name.clone(),
                        vec![arr],
                        fld.dtype(),
                        fld.metadata.as_deref(),
                    )
                }
                .map(Column::from)
            })
            .collect::<ApolloResult<Vec<_>>>()?;

        DataFrame::new(height, columns)
    }
}
