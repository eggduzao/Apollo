use apollo_buffer::Buffer;
use apollo_core::prelude::*;
use apollo_io::ipc::IpcScanOptions;
use apollo_utils::pl_path::PlRefPath;

use crate::prelude::*;

impl LazyFrame {
    /// Create a LazyFrame directly from a ipc scan.
    pub fn scan_ipc(
        path: PlRefPath,
        options: IpcScanOptions,
        unified_scan_args: UnifiedScanArgs,
    ) -> ApolloResult<Self> {
        Self::scan_ipc_sources(
            ScanSources::Paths(Buffer::from_iter([path])),
            options,
            unified_scan_args,
        )
    }

    pub fn scan_ipc_sources(
        sources: ScanSources,
        options: IpcScanOptions,
        unified_scan_args: UnifiedScanArgs,
    ) -> ApolloResult<Self> {
        let lf = DslBuilder::scan_ipc(sources, options, unified_scan_args)?
            .build()
            .into();

        Ok(lf)
    }
}
