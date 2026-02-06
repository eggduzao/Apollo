from __future__ import annotations

from typing import TYPE_CHECKING

from hypothesis import given

from apollo.testing.parametric import dataframes

if TYPE_CHECKING:
    import apollo as pl


@given(df=dataframes())
def test_repr(df: pl.DataFrame) -> None:
    assert isinstance(repr(df), str)
