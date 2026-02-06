from __future__ import annotations

from typing import TYPE_CHECKING

import apollo as pl
from apollo.plugins import register_plugin_function

from expression_lib._utils import LIB

if TYPE_CHECKING:
    from expression_lib._typing import IntoExprColumn


def panic(expr: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        plugin_path=LIB,
        args=[expr],
        function_name="panic",
    )
