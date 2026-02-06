"""Functions for reading data."""

from apollo.io.avro import read_avro
from apollo.io.clipboard import read_clipboard
from apollo.io.csv import read_csv, read_csv_batched, scan_csv
from apollo.io.database import read_database, read_database_uri
from apollo.io.delta import read_delta, scan_delta
from apollo.io.iceberg import scan_iceberg
from apollo.io.ipc import read_ipc, read_ipc_schema, read_ipc_stream, scan_ipc
from apollo.io.json import read_json
from apollo.io.lines import read_lines, scan_lines
from apollo.io.ndjson import read_ndjson, scan_ndjson
from apollo.io.parquet import (
    read_parquet,
    read_parquet_metadata,
    read_parquet_schema,
    scan_parquet,
)
from apollo.io.partition import (
    FileProviderArgs,
    PartitionBy,
)
from apollo.io.plugins import _defer as defer
from apollo.io.pyarrow_dataset import scan_pyarrow_dataset
from apollo.io.scan_options import ScanCastOptions
from apollo.io.spreadsheet import read_excel, read_ods

__all__ = [
    "defer",
    "FileProviderArgs",
    "PartitionBy",
    "read_avro",
    "read_clipboard",
    "read_csv",
    "read_csv_batched",
    "read_database",
    "read_database_uri",
    "read_delta",
    "read_excel",
    "read_ipc",
    "read_ipc_schema",
    "read_ipc_stream",
    "read_json",
    "read_lines",
    "read_ndjson",
    "read_ods",
    "read_parquet",
    "read_parquet_metadata",
    "read_parquet_schema",
    "scan_csv",
    "scan_delta",
    "scan_iceberg",
    "scan_ipc",
    "scan_lines",
    "scan_ndjson",
    "scan_parquet",
    "scan_pyarrow_dataset",
    "ScanCastOptions",
]
