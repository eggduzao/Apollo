use std::sync::Arc;

use arrow::datatypes::ArrowSchemaRef;
use apollo_buffer::Buffer;
use apollo_error::ApolloResult;
use apollo_io::parquet::write::BatchedWriter;
use apollo_io::prelude::KeyValueMetadata;
use apollo_parquet::write::{Encoding, FileWriter, SchemaDescriptor, WriteOptions};

use crate::async_executor::{self};
use crate::nodes::io_sinks::writers::interface::FileOpenTaskHandle;
use crate::nodes::io_sinks::writers::parquet::EncodedRowGroup;

pub struct IOWriter {
    pub file: FileOpenTaskHandle,
    pub encoded_row_group_rx: tokio::sync::mpsc::Receiver<
        async_executor::AbortOnDropHandle<ApolloResult<EncodedRowGroup>>,
    >,
    pub arrow_schema: ArrowSchemaRef,
    pub schema_descriptor: Arc<SchemaDescriptor>,
    pub write_options: WriteOptions,
    pub encodings: Buffer<Vec<Encoding>>,
    pub key_value_metadata: Option<KeyValueMetadata>,
    pub num_leaf_columns: usize,
}

impl IOWriter {
    pub async fn run(self) -> ApolloResult<()> {
        let IOWriter {
            file,
            mut encoded_row_group_rx,
            arrow_schema,
            schema_descriptor,
            write_options,
            encodings,
            key_value_metadata,
            num_leaf_columns,
        } = self;

        let (mut file, sync_on_close) = file.await?;
        let mut buffered_file = file.as_buffered();

        let mut parquet_writer = BatchedWriter::new(
            std::sync::Mutex::new(FileWriter::new_with_parquet_schema(
                &mut *buffered_file,
                Arc::unwrap_or_clone(arrow_schema),
                Arc::unwrap_or_clone(schema_descriptor),
                write_options,
            )),
            encodings,
            write_options,
            false,
            key_value_metadata,
        );

        while let Some(handle) = encoded_row_group_rx.recv().await {
            let EncodedRowGroup {
                num_rows,
                data,
                morsel_permit,
            } = handle.await?;
            assert_eq!(data.len(), num_leaf_columns);
            parquet_writer.write_row_group(num_rows as u64, &data)?;
            drop(data);
            drop(morsel_permit);
        }

        parquet_writer.finish()?;
        drop(parquet_writer);
        drop(buffered_file);

        file.close(sync_on_close)?;

        Ok(())
    }
}
