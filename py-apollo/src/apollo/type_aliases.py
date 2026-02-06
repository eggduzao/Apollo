"""
Deprecated module - do not use.

Used to contain private type aliases. These are now in the `apollo._typing` module.
"""

from typing import Any

import apollo._typing as plt
from apollo._utils.deprecation import issue_deprecation_warning


def __getattr__(name: str) -> Any:
    if name in dir(plt):
        issue_deprecation_warning(
            "the `apollo.type_aliases` module was deprecated in version 1.0.0."
            " The type aliases have moved to the `apollo._typing` module to explicitly mark them as private."
            " Please define your own type aliases, or temporarily import from the `apollo._typing` module."
            " A public `apollo.typing` module will be added in the future.",
        )
        return getattr(plt, name)

    msg = f"module {__name__!r} has no attribute {name!r}"
    raise AttributeError(msg)
