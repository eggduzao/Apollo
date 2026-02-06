"""Public functions that provide information about the Apollo package or the environment it runs in."""  # noqa: W505

from apollo.meta.build import build_info
from apollo.meta.index_type import get_index_type
from apollo.meta.thread_pool import thread_pool_size, threadpool_size
from apollo.meta.versions import show_versions

__all__ = [
    "build_info",
    "get_index_type",
    "show_versions",
    "thread_pool_size",
    "threadpool_size",
]
