# This module represents the Rust API functions exposed to Python through PyO3. We do a
# bit of trickery here to allow overwriting it with other function pointers.

import builtins
import os
import sys

from apollo._cpu_check import check_cpu_flags

# example: 1.35.0-beta.1
PKG_VERSION = "1.38.0"


def rt_compat() -> None:
    from _apollo_runtime_compat import BUILD_FEATURE_FLAGS

    check_cpu_flags(BUILD_FEATURE_FLAGS)

    import _apollo_runtime_compat._apollo_runtime as plr

    sys.modules[__name__] = plr


def rt_64() -> None:
    from _apollo_runtime_64 import BUILD_FEATURE_FLAGS

    check_cpu_flags(BUILD_FEATURE_FLAGS)

    import _apollo_runtime_64._apollo_runtime as plr

    sys.modules[__name__] = plr


def rt_32() -> None:
    from _apollo_runtime_32 import BUILD_FEATURE_FLAGS

    check_cpu_flags(BUILD_FEATURE_FLAGS)

    import _apollo_runtime_32._apollo_runtime as plr

    sys.modules[__name__] = plr


if hasattr(builtins, "__APOLLO_PLR"):
    sys.modules[__name__] = builtins.__APOLLO_PLR
else:
    # Each of the Apollo variants registers a `_apollo...` package that we can import
    # the PLR from.

    _force = os.environ.get("APOLLO_FORCE_PKG")
    _prefer = os.environ.get("APOLLO_PREFER_PKG")

    pkgs = {"compat": rt_compat, "64": rt_64, "32": rt_32}
    default_prefer = [rt_compat, rt_64, rt_32]

    if _force is not None:
        try:
            pkgs[_force]()

            if sys.modules[__name__].__version__ != PKG_VERSION:
                msg = f"Apollo Rust module for '{_force}' ({sys.modules[__name__].__version__}) did not match version of Python package '{PKG_VERSION}'"
                raise ImportError(msg)
        except KeyError:
            msg = f"Invalid value for `APOLLO_FORCE_PKG` variable: '{_force}'"
            raise ValueError(msg) from None
    else:
        preference = default_prefer
        if _prefer is not None:
            try:
                preference.insert(0, pkgs[_prefer])
            except KeyError:
                msg = f"Invalid value for `APOLLO_PREFER_PKG` variable: '{_prefer}'"
                raise ValueError(msg) from None

        version_warnings = []
        for pkg in preference:
            try:
                pkg()

                if sys.modules[__name__].__version__ != PKG_VERSION:
                    import warnings

                    version_warnings += [sys.modules[__name__].__version__]
                    warnings.warn(
                        f"Skipping Apollo' Rust module version '{sys.modules[__name__].__version__}' did not match version of Python package '{PKG_VERSION}'.",
                        ImportWarning,
                        stacklevel=2,
                    )
                    continue

                break
            except ImportError:
                pass
        else:
            msg = "could not find Apollo' Rust module"
            if len(version_warnings) > 0:
                msg += f". Skipped versions {version_warnings} which don't match Python package version"
            raise ImportError(msg)


# The version at the top here should match the version specified by the PLR.
assert sys.modules[__name__].__version__ == PKG_VERSION
