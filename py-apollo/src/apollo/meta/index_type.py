from __future__ import annotations

import contextlib
from typing import TYPE_CHECKING

with contextlib.suppress(ImportError):  # Module not available when building docs
    import apollo._plr as plr

if TYPE_CHECKING:
    from apollo._typing import ApolloIntegerType


def get_index_type() -> ApolloIntegerType:
    """
    Return the data type used for Apollo indexing.

    Returns
    -------
    ApolloIntegerType
        :class:`UInt32` in regular Apollo, :class:`UInt64` in bigidx Apollo.

    Examples
    --------
    >>> pl.get_index_type()
    UInt32
    """
    return plr.get_index_type()
