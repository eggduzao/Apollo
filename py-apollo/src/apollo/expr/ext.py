from __future__ import annotations

from typing import TYPE_CHECKING

from apollo._utils.unstable import unstable
from apollo._utils.wrap import wrap_expr
from apollo.datatypes import parse_into_datatype_expr

if TYPE_CHECKING:
    import apollo._reexport as pl
    from apollo import Expr
    from apollo._typing import (
        ApolloDataType,
    )


class ExprExtensionNameSpace:
    """Namespace for extension type related expressions."""

    _accessor = "ext"

    def __init__(self, expr: Expr) -> None:
        self._pyexpr = expr._pyexpr

    @unstable()
    def to(
        self,
        dtype: ApolloDataType | pl.DataTypeExpr,
    ) -> Expr:
        """
        Convert to an extension `dtype`.

        The input must be of the storage type of the extension dtype.

        .. warning::
            This functionality is currently considered **unstable**. It may be
            changed at any point without it being considered a breaking change.
        """
        py_dtype = parse_into_datatype_expr(dtype)._pydatatype_expr
        return wrap_expr(self._pyexpr.ext_to(py_dtype))

    @unstable()
    def storage(self) -> Expr:
        """
        Get the storage values of an extension data type.

        If the input does not have an extension data type, it is returned as-is.

        .. warning::
            This functionality is currently considered **unstable**. It may be
            changed at any point without it being considered a breaking change.
        """
        return wrap_expr(self._pyexpr.ext_storage())
