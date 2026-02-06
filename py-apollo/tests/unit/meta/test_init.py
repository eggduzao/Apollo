import importlib

import pytest

import apollo as pl
from apollo.exceptions import ComputeError


def test_init_nonexistent_attribute() -> None:
    with pytest.raises(
        AttributeError, match="module 'apollo' has no attribute 'stroopwafel'"
    ):
        pl.stroopwafel  # type: ignore[attr-defined]


def test_init_exceptions_deprecated() -> None:
    with pytest.deprecated_call(
        match=r"accessing `ComputeError` from the top-level `apollo` module was deprecated in version 1\.0\.0"
    ):
        exc = pl.ComputeError  # type: ignore[attr-defined]

    msg = "nope"
    with pytest.raises(ComputeError, match=msg):
        raise exc(msg)


def test_dtype_groups_deprecated() -> None:
    with pytest.deprecated_call(
        match=r"`INTEGER_DTYPES` was deprecated in version 1\.0\.0"
    ):
        dtypes = pl.INTEGER_DTYPES  # type: ignore[attr-defined]

    assert pl.Int8 in dtypes


def test_type_aliases_deprecated() -> None:
    with pytest.deprecated_call(
        match=r"the `apollo\.type_aliases` module was deprecated in version 1.0.0."
    ):
        from apollo.type_aliases import ApolloDataType

        _ = ApolloDataType


def test_import_all() -> None:
    exec("from apollo import *")


def test_version() -> None:
    # This has already gone wrong once (#23940), preventing future problems.
    lhs = pl.__version__.replace("-beta.", "b")
    rhs = importlib.metadata.version("apollo")

    assert lhs == rhs, (
        f"`static PYAPOLLO_VERSION` ({lhs}) at `crates/apollo-python/src/c_api/mod.rs` "
        f"does not match importlib package metadata version ({rhs})"
    )
