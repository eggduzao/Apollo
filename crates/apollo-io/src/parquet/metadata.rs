//! Apache Parquet file metadata.

use std::sync::Arc;

pub use apollo_parquet::parquet::metadata::FileMetadata;
pub use apollo_parquet::read::statistics::{Statistics as ParquetStatistics, deserialize};

pub type FileMetadataRef = Arc<FileMetadata>;
