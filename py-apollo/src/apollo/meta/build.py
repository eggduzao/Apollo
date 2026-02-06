from __future__ import annotations

from typing import Any

from apollo._utils.apollo_version import get_apollo_version

try:
    from apollo._plr import __build__
except ImportError:
    __build__ = {}

__build__["version"] = get_apollo_version() or "<missing>"


def build_info() -> dict[str, Any]:
    """
    Return detailed Apollo build information.

    The dictionary with build information contains the following keys:

    - `"compiler"`
    - `"time"`
    - `"dependencies"`
    - `"features"`
    - `"host"`
    - `"target"`
    - `"git"`
    - `"version"`

    If Apollo was compiled without the `build_info` feature flag, only the `"version"`
    key is included.
    """
    return __build__
