use apollo_core::error::{ApolloResult, apollo_err};

use crate::constants::APOLLO_PLACEHOLDER;
use crate::dsl::{DslPlan, FileScanDsl, ScanSources, SinkType};

/// Assert that the given [`DslPlan`] is eligible to be executed on Apollo Cloud.
pub(super) fn assert_cloud_eligible(dsl: &DslPlan, allow_local_scans: bool) -> ApolloResult<()> {
    if std::env::var("APOLLO_SKIP_CLIENT_CHECK").as_deref() == Ok("1") {
        return Ok(());
    }

    // Check that the plan ends with a sink.
    if !matches!(dsl, DslPlan::Sink { .. } | DslPlan::SinkMultiple { .. }) {
        return ineligible_error("does not contain a sink");
    }

    for plan_node in dsl.into_iter() {
        match plan_node {
            #[cfg(feature = "python")]
            DslPlan::PythonScan { .. } => (),
            DslPlan::Scan {
                sources, scan_type, ..
            } => {
                match sources {
                    ScanSources::Paths(paths) => {
                        if !allow_local_scans
                            && paths
                                .iter()
                                .any(|p| !p.has_scheme() && p.as_str() != APOLLO_PLACEHOLDER)
                        {
                            return ineligible_error("contains scan of local file system");
                        }
                    },
                    ScanSources::Files(_) => {
                        return ineligible_error("contains scan of opened files");
                    },
                    ScanSources::Buffers(_) => {
                        return ineligible_error("contains scan of in-memory buffer");
                    },
                }

                if matches!(&**scan_type, FileScanDsl::Anonymous { .. }) {
                    return ineligible_error("contains anonymous scan");
                }
            },
            DslPlan::Sink { payload, .. } => {
                match payload {
                    SinkType::Memory => {
                        return ineligible_error("contains memory sink");
                    },
                    SinkType::Callback(_) => {
                        return ineligible_error("contains callback sink");
                    },
                    SinkType::File { .. } | SinkType::Partitioned { .. } => {
                        // The sink destination is passed around separately, can't check the
                        // eligibility here.
                    },
                }
            },
            _ => (),
        }
    }
    Ok(())
}

fn ineligible_error(message: &str) -> ApolloResult<()> {
    Err(apollo_err!(
        InvalidOperation:
        "logical plan ineligible for execution on Apollo Cloud: {message}"
    ))
}
