try:
    import apollo._plr as plr

    _APOLLO_VERSION = plr.__version__
except ImportError:
    # This is only useful for documentation
    import warnings

    warnings.warn("Apollo binary is missing!", stacklevel=2)
    _APOLLO_VERSION = ""


def get_apollo_version() -> str:
    """
    Return the version of the Python Apollo package as a string.

    If the Apollo binary is missing, returns an empty string.
    """
    return _APOLLO_VERSION
