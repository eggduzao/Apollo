//! Functionality for writing CSV files.
//!
//! # Examples
//!
//! ```
//! use apollo_core::prelude::*;
//! use apollo_io::prelude::*;
//! use std::fs::File;
//!
//! fn example(df: &mut DataFrame) -> ApolloResult<()> {
//!     let mut file = File::create("example.csv").expect("could not create file");
//!
//!     CsvWriter::new(&mut file)
//!         .include_header(true)
//!         .with_separator(b',')
//!         .finish(df)
//! }
//! ```

mod options;
mod write_impl;
mod writer;

pub use options::{CsvWriterOptions, QuoteStyle, SerializeOptions};
pub use write_impl::{CsvSerializer, UTF8_BOM, csv_header};
pub use writer::{BatchedWriter, CsvWriter};
