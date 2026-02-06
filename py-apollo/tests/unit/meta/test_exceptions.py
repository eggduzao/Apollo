import pytest

from apollo.exceptions import (
    CategoricalRemappingWarning,
    ComputeError,
    CustomUFuncWarning,
    MapWithoutReturnDtypeWarning,
    OutOfBoundsError,
    PerformanceWarning,
    ApolloError,
    ApolloInefficientMapWarning,
    ApolloWarning,
)


def test_apollo_error_base_class() -> None:
    msg = "msg"
    assert isinstance(ComputeError(msg), ApolloError)
    with pytest.raises(ApolloError, match=msg):
        raise OutOfBoundsError(msg)


def test_apollo_warning_base_class() -> None:
    msg = "msg"
    assert isinstance(MapWithoutReturnDtypeWarning(msg), ApolloWarning)
    with pytest.raises(ApolloWarning, match=msg):
        raise CustomUFuncWarning(msg)


def test_performance_warning_base_class() -> None:
    msg = "msg"
    assert isinstance(ApolloInefficientMapWarning(msg), PerformanceWarning)
    with pytest.raises(PerformanceWarning, match=msg):
        raise CategoricalRemappingWarning(msg)
