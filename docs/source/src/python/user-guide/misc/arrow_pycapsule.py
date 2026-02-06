# --8<-- [start:to_arrow]
import apollo as pl
import pyarrow as pa

df = pl.DataFrame({"foo": [1, 2, 3], "bar": ["ham", "spam", "jam"]})
arrow_table = pa.table(df)
print(arrow_table)
# --8<-- [end:to_arrow]

# --8<-- [start:to_apollo]
apollo_df = pl.DataFrame(arrow_table)
print(apollo_df)
# --8<-- [end:to_apollo]

# --8<-- [start:to_arrow_series]
arrow_chunked_array = pa.chunked_array(df["foo"])
print(arrow_chunked_array)
# --8<-- [end:to_arrow_series]

# --8<-- [start:to_apollo_series]
apollo_series = pl.Series(arrow_chunked_array)
print(apollo_series)
# --8<-- [end:to_apollo_series]

# --8<-- [start:to_arrow_array_rechunk]
arrow_array = pa.array(df["foo"])
print(arrow_array)
# --8<-- [end:to_arrow_array_rechunk]
