import pytest

import apollo as pl
from apollo._dependencies import _lazy_import
from apollo.testing import assert_frame_equal

# don't import apollo_ds until an actual test is triggered (the decorator already
# ensures the tests aren't run locally; this avoids premature local import)
pds, _ = _lazy_import("apollo_ds")

pytestmark = pytest.mark.ci_only


def test_basic_operation() -> None:
    # We are mostly interested in making sure that we can actually still call the plugin
    # properly.
    df = pl.DataFrame({"name": ["a", "b", "c"]})
    assert_frame_equal(
        df.select(pds.str_leven("name", pl.lit("che"))),
        pl.Series("name", [3, 3, 2], pl.UInt32).to_frame(),
    )
