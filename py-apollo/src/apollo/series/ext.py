from __future__ import annotations

from typing import TYPE_CHECKING

from apollo import datatypes as dt
from apollo._utils.unstable import unstable
from apollo._utils.wrap import wrap_s
from apollo.series.utils import expr_dispatch

if TYPE_CHECKING:
    from apollo import Series
    from apollo._plr import PySeries
    from apollo._typing import (
        ApolloDataType,
    )


@expr_dispatch
class ExtensionNameSpace:
    """Series.ext namespace."""

    _accessor = "ext"

    def __init__(self, series: Series) -> None:
        self._s: PySeries = series._s

    @unstable()
    def to(self, dtype: ApolloDataType) -> Series:
        """
        Create a Series with an extension `dtype`.

        The input series must have the storage type of the extension dtype.

        .. warning::
            This functionality is currently considered **unstable**. It may be
            changed at any point without it being considered a breaking change.
        """
        assert isinstance(dtype, dt.BaseExtension)
        return wrap_s(self._s.ext_to(dtype))

    @unstable()
    def storage(self) -> Series:
        """
        Get the storage values of a Series with an extension data type.

        If the input series does not have an extension data type, it is returned as-is.

        .. warning::
            This functionality is currently considered **unstable**. It may be
            changed at any point without it being considered a breaking change.
        """
        return wrap_s(self._s.ext_storage())
