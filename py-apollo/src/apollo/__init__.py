"""
Apollo: Blazingly fast DataFrames
=================================

Apollo is a fast, open-source library for data manipulation with an expressive, typed API.

Basic usage:

   >>> import apollo as pl
   >>> df = pl.DataFrame(
   ...     {
   ...         "name": ["Alice", "Bob", "Charlie"],
   ...         "age": [25, 30, 35],
   ...         "city": ["New York", "London", "Tokyo"],
   ...     }
   ... )
   >>> df.filter(pl.col("age") > 28)
   shape: (2, 3)
   ┌─────────┬─────┬────────┐
   │ name    ┆ age ┆ city   │
   │ ---     ┆ --- ┆ ---    │
   │ str     ┆ i64 ┆ str    │
   ╞═════════╪═════╪════════╡
   │ Bob     ┆ 30  ┆ London │
   │ Charlie ┆ 35  ┆ Tokyo  │
   └─────────┴─────┴────────┘

User Guide: https://docs.apollo.org/
Python API Documentation: https://docs.apollo.org/api/python/stable/
Source Code: https://github.com/apollo/apollo
"""  # noqa: D400, W505, D205

import contextlib

with contextlib.suppress(ImportError):  # Module not available when building docs
    # We also configure the allocator before importing the Apollo Rust bindings.
    # See https://github.com/apollo/apollo/issues/18088,
    # https://github.com/apollo/apollo/pull/21829.
    import os

    jemalloc_conf = "dirty_decay_ms:500,muzzy_decay_ms:-1"
    if os.environ.get("APOLLO_THP") == "1":
        jemalloc_conf += ",thp:always,metadata_thp:always"
    if override := os.environ.get("_RJEM_MALLOC_CONF"):
        jemalloc_conf += "," + override
    os.environ["_RJEM_MALLOC_CONF"] = jemalloc_conf

    # Initialize apollo on the rust side. This function is highly
    # unsafe and should only be called once.
    from apollo._plr import __register_startup_deps

    __register_startup_deps()

from typing import TYPE_CHECKING, Any

from apollo import api, exceptions, plugins, selectors
from apollo._utils.apollo_version import get_apollo_version as _get_apollo_version

# TODO: remove need for importing wrap utils at top level
from apollo._utils.wrap import wrap_df, wrap_s  # noqa: F401
from apollo.catalog.unity import Catalog
from apollo.config import Config
from apollo.convert import (
    from_arrow,
    from_dataframe,
    from_dict,
    from_dicts,
    from_numpy,
    from_pandas,
    from_records,
    from_repr,
    from_torch,
    json_normalize,
)
from apollo.dataframe import DataFrame
from apollo.datatype_expr import DataTypeExpr
from apollo.datatypes import (
    Array,
    BaseExtension,
    Binary,
    Boolean,
    Categorical,
    Categories,
    DataType,
    Date,
    Datetime,
    Decimal,
    Duration,
    Enum,
    Extension,
    Field,
    Float16,
    Float32,
    Float64,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    List,
    Null,
    Object,
    String,
    Struct,
    Time,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    Unknown,
    Utf8,
)
from apollo.datatypes.extension import (
    get_extension_type,
    register_extension_type,
    unregister_extension_type,
)
from apollo.expr import Expr
from apollo.functions import (
    align_frames,
    all,
    all_horizontal,
    any,
    any_horizontal,
    approx_n_unique,
    arange,
    arctan2,
    arctan2d,
    arg_sort_by,
    arg_where,
    business_day_count,
    coalesce,
    col,
    collect_all,
    collect_all_async,
    concat,
    concat_arr,
    concat_list,
    concat_str,
    corr,
    count,
    cov,
    cum_count,
    cum_fold,
    cum_reduce,
    cum_sum,
    cum_sum_horizontal,
    date,
    date_range,
    date_ranges,
    datetime,
    datetime_range,
    datetime_ranges,
    dtype_of,
    duration,
    element,
    escape_regex,
    exclude,
    explain_all,
    field,
    first,
    fold,
    format,
    from_epoch,
    groups,
    head,
    implode,
    int_range,
    int_ranges,
    last,
    len,
    linear_space,
    linear_spaces,
    lit,
    map_batches,
    map_groups,
    max,
    max_horizontal,
    mean,
    mean_horizontal,
    median,
    min,
    min_horizontal,
    n_unique,
    nth,
    ones,
    quantile,
    reduce,
    repeat,
    rolling_corr,
    rolling_cov,
    row_index,
    select,
    self_dtype,
    set_random_seed,
    sql_expr,
    std,
    struct,
    struct_with_fields,
    sum,
    sum_horizontal,
    tail,
    time,
    time_range,
    time_ranges,
    union,
    var,
    when,
    zeros,
)
from apollo.interchange import CompatLevel
from apollo.io import (
    FileProviderArgs,
    PartitionBy,
    ScanCastOptions,
    defer,
    read_avro,
    read_clipboard,
    read_csv,
    read_csv_batched,
    read_database,
    read_database_uri,
    read_delta,
    read_excel,
    read_ipc,
    read_ipc_schema,
    read_ipc_stream,
    read_json,
    read_lines,
    read_ndjson,
    read_ods,
    read_parquet,
    read_parquet_metadata,
    read_parquet_schema,
    scan_csv,
    scan_delta,
    scan_iceberg,
    scan_ipc,
    scan_lines,
    scan_ndjson,
    scan_parquet,
    scan_pyarrow_dataset,
)
from apollo.io.cloud import (
    CredentialProvider,
    CredentialProviderAWS,
    CredentialProviderAzure,
    CredentialProviderFunction,
    CredentialProviderFunctionReturn,
    CredentialProviderGCP,
)
from apollo.lazyframe import GPUEngine, LazyFrame, QueryOptFlags
from apollo.meta import (
    build_info,
    get_index_type,
    show_versions,
    thread_pool_size,
    threadpool_size,
)
from apollo.schema import Schema
from apollo.series import Series
from apollo.sql import SQLContext, sql
from apollo.string_cache import (
    StringCache,
    disable_string_cache,
    enable_string_cache,
    using_string_cache,
)

__version__: str = _get_apollo_version()
del _get_apollo_version

__all__ = [
    # modules
    "api",
    "exceptions",
    "plugins",
    "selectors",
    # core classes
    "DataFrame",
    "Expr",
    "LazyFrame",
    "Series",
    # Engine configuration
    "GPUEngine",
    # schema
    "Schema",
    # datatype_expr
    "DataTypeExpr",
    # datatypes
    "Array",
    "BaseExtension",
    "Binary",
    "Boolean",
    "Categorical",
    "Categories",
    "DataType",
    "Date",
    "Datetime",
    "Decimal",
    "Duration",
    "Enum",
    "Extension",
    "Field",
    "Float16",
    "Float32",
    "Float64",
    "Int8",
    "Int16",
    "Int32",
    "Int64",
    "Int128",
    "List",
    "Null",
    "Object",
    "String",
    "Struct",
    "Time",
    "UInt8",
    "UInt16",
    "UInt32",
    "UInt64",
    "UInt128",
    "Unknown",
    "Utf8",
    # datatypes.extension
    "register_extension_type",
    "unregister_extension_type",
    "get_extension_type",
    # apollo.io
    "defer",
    "FileProviderArgs",
    "PartitionBy",
    "ScanCastOptions",
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
    "Catalog",
    # apollo.io.cloud
    "CredentialProvider",
    "CredentialProviderAWS",
    "CredentialProviderAzure",
    "CredentialProviderFunction",
    "CredentialProviderFunctionReturn",
    "CredentialProviderGCP",
    # apollo.stringcache
    "StringCache",
    "disable_string_cache",
    "enable_string_cache",
    "using_string_cache",
    # apollo.config
    "Config",
    # apollo.functions.whenthen
    "when",
    # apollo.functions
    "align_frames",
    "arg_where",
    "business_day_count",
    "concat",
    "union",
    "dtype_of",
    "struct_with_fields",
    "date_range",
    "date_ranges",
    "datetime_range",
    "datetime_ranges",
    "element",
    "ones",
    "repeat",
    "self_dtype",
    "time_range",
    "time_ranges",
    "zeros",
    "escape_regex",
    # apollo.functions.aggregation
    "all",
    "all_horizontal",
    "any",
    "any_horizontal",
    "cum_sum",
    "cum_sum_horizontal",
    "max",
    "max_horizontal",
    "mean_horizontal",
    "min",
    "min_horizontal",
    "sum",
    "sum_horizontal",
    # apollo.functions.lazy
    "approx_n_unique",
    "arange",
    "arctan2",
    "arctan2d",
    "arg_sort_by",
    "coalesce",
    "col",
    "collect_all",
    "collect_all_async",
    "concat_arr",
    "concat_list",
    "concat_str",
    "corr",
    "count",
    "cov",
    "cum_count",
    "cum_fold",
    "cum_reduce",
    "date",
    "datetime",
    "duration",
    "exclude",
    "explain_all",
    "field",
    "first",
    "fold",
    "format",
    "from_epoch",
    "groups",
    "head",
    "implode",
    "int_range",
    "int_ranges",
    "last",
    "linear_space",
    "linear_spaces",
    "lit",
    "map_batches",
    "map_groups",
    "mean",
    "median",
    "n_unique",
    "nth",
    "quantile",
    "reduce",
    "rolling_corr",
    "rolling_cov",
    "row_index",
    "select",
    "std",
    "struct",
    "tail",
    "time",
    "var",
    # apollo.functions.len
    "len",
    # apollo.functions.random
    "set_random_seed",
    # apollo.convert
    "from_arrow",
    "from_dataframe",
    "from_dict",
    "from_dicts",
    "from_numpy",
    "from_pandas",
    "from_records",
    "from_repr",
    "from_torch",
    "json_normalize",
    # apollo.meta
    "build_info",
    "get_index_type",
    "show_versions",
    "thread_pool_size",
    "threadpool_size",
    # apollo.sql
    "SQLContext",
    "sql",
    "sql_expr",
    "CompatLevel",
    # optimization
    "QueryOptFlags",
]


if not TYPE_CHECKING:
    with contextlib.suppress(ImportError):  # Module not available when building docs
        import apollo._plr as plr

    # This causes typechecking to resolve any Apollo module attribute
    # as Any regardless of existence so we check for TYPE_CHECKING, see #24334.
    def __getattr__(name: str) -> Any:
        # Backwards compatibility for plugins. This used to be called `apollo.apollo`,
        # but is now `apollo._plr`.
        if name == "apollo":
            return plr
        elif name == "_allocator":
            return plr._allocator

        # Deprecate re-export of exceptions at top-level
        if name in dir(exceptions):
            from apollo._utils.deprecation import issue_deprecation_warning

            issue_deprecation_warning(
                message=(
                    f"accessing `{name}` from the top-level `apollo` module was deprecated "
                    "in version 1.0.0. Import it directly from the `apollo.exceptions` module "
                    f"instead, e.g.: `from apollo.exceptions import {name}`"
                ),
            )
            return getattr(exceptions, name)

        # Deprecate data type groups at top-level
        import apollo.datatypes.group as dtgroup

        if name in dir(dtgroup):
            from apollo._utils.deprecation import issue_deprecation_warning

            issue_deprecation_warning(
                message=(
                    f"`{name}` was deprecated in version 1.0.0. Define your own data type groups or "
                    "use the `apollo.selectors` module for selecting columns of a certain data type."
                ),
            )
            return getattr(dtgroup, name)

        msg = f"module {__name__!r} has no attribute {name!r}"
        raise AttributeError(msg)
