use std::str::FromStr;

#[cfg(any(
    feature = "csv",
    feature = "parquet",
    feature = "ipc",
    feature = "json"
))]
use apollo_core::prelude::apollo_ensure;
use apollo_core::prelude::{ApolloError, ApolloResult, apollo_bail};
#[cfg(feature = "csv")]
use apollo_lazy::prelude::LazyCsvReader;
use apollo_lazy::prelude::LazyFrame;
use apollo_utils::pl_path::PlRefPath;
use sqlparser::ast::{
    Expr as SQLExpr, FunctionArg as SQLFunctionArg, FunctionArgExpr as SQLFunctionArgExpr,
    Value as SQLValue, ValueWithSpan as SQLValueWithSpan,
};

/// Table functions that are supported by Apollo
#[allow(clippy::enum_variant_names)]
pub(crate) enum ApolloTableFunctions {
    /// SQL 'read_csv' function.
    /// ```sql
    /// SELECT * FROM read_csv('path/to/file.csv')
    /// ```
    #[cfg(feature = "csv")]
    ReadCsv,
    /// SQL 'read_parquet' function.
    /// ```sql
    /// SELECT * FROM read_parquet('path/to/file.parquet')
    /// ```
    #[cfg(feature = "parquet")]
    ReadParquet,
    /// SQL 'read_ipc' function.
    /// ```sql
    /// SELECT * FROM read_ipc('path/to/file.ipc')
    /// ```
    #[cfg(feature = "ipc")]
    ReadIpc,
    /// SQL 'read_json' function (*only ndjson is currently supported*).
    /// ```sql
    /// SELECT * FROM read_json('path/to/file.json')
    /// ```
    #[cfg(feature = "json")]
    ReadJson,
}

impl FromStr for ApolloTableFunctions {
    type Err = ApolloError;

    #[allow(unreachable_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            #[cfg(feature = "csv")]
            "read_csv" => ApolloTableFunctions::ReadCsv,
            #[cfg(feature = "parquet")]
            "read_parquet" => ApolloTableFunctions::ReadParquet,
            #[cfg(feature = "ipc")]
            "read_ipc" => ApolloTableFunctions::ReadIpc,
            #[cfg(feature = "json")]
            "read_json" => ApolloTableFunctions::ReadJson,
            _ => apollo_bail!(SQLInterface: "'{}' is not a supported table function", s),
        })
    }
}

impl ApolloTableFunctions {
    #[allow(unused_variables, unreachable_patterns)]
    pub(crate) fn execute(&self, args: &[SQLFunctionArg]) -> ApolloResult<(PlRefPath, LazyFrame)> {
        match self {
            #[cfg(feature = "csv")]
            ApolloTableFunctions::ReadCsv => self.read_csv(args),
            #[cfg(feature = "parquet")]
            ApolloTableFunctions::ReadParquet => self.read_parquet(args),
            #[cfg(feature = "ipc")]
            ApolloTableFunctions::ReadIpc => self.read_ipc(args),
            #[cfg(feature = "json")]
            ApolloTableFunctions::ReadJson => self.read_ndjson(args),
            _ => unreachable!(),
        }
    }

    #[cfg(feature = "csv")]
    fn read_csv(&self, args: &[SQLFunctionArg]) -> ApolloResult<(PlRefPath, LazyFrame)> {
        apollo_ensure!(args.len() == 1, SQLSyntax: "`read_csv` expects a single file path; found {:?} arguments", args.len());

        use apollo_lazy::frame::LazyFileListReader;
        let path = self.get_file_path_from_arg(&args[0])?;
        let lf = LazyCsvReader::new(path.clone())
            .with_try_parse_dates(true)
            .with_missing_is_null(true)
            .finish()?;
        Ok((path, lf))
    }

    #[cfg(feature = "parquet")]
    fn read_parquet(&self, args: &[SQLFunctionArg]) -> ApolloResult<(PlRefPath, LazyFrame)> {
        apollo_ensure!(args.len() == 1, SQLSyntax: "`read_parquet` expects a single file path; found {:?} arguments", args.len());

        let path = self.get_file_path_from_arg(&args[0])?;
        let lf = LazyFrame::scan_parquet(path.clone(), Default::default())?;
        Ok((path, lf))
    }

    #[cfg(feature = "ipc")]
    fn read_ipc(&self, args: &[SQLFunctionArg]) -> ApolloResult<(PlRefPath, LazyFrame)> {
        apollo_ensure!(args.len() == 1, SQLSyntax: "`read_ipc` expects a single file path; found {:?} arguments", args.len());

        let path = self.get_file_path_from_arg(&args[0])?;
        let lf = LazyFrame::scan_ipc(path.clone(), Default::default(), Default::default())?;
        Ok((path, lf))
    }
    #[cfg(feature = "json")]
    fn read_ndjson(&self, args: &[SQLFunctionArg]) -> ApolloResult<(PlRefPath, LazyFrame)> {
        apollo_ensure!(args.len() == 1, SQLSyntax: "`read_ndjson` expects a single file path; found {:?} arguments", args.len());

        use apollo_lazy::frame::LazyFileListReader;
        use apollo_lazy::prelude::LazyJsonLineReader;

        let path = self.get_file_path_from_arg(&args[0])?;
        let lf = LazyJsonLineReader::new(path.clone()).finish()?;
        Ok((path, lf))
    }

    #[allow(dead_code)]
    fn get_file_path_from_arg(&self, arg: &SQLFunctionArg) -> ApolloResult<PlRefPath> {
        match arg {
            SQLFunctionArg::Unnamed(SQLFunctionArgExpr::Expr(SQLExpr::Value(
                SQLValueWithSpan {
                    value: SQLValue::SingleQuotedString(s),
                    ..
                },
            ))) => Ok(PlRefPath::new(s)),
            _ => apollo_bail!(
                SQLSyntax:
                "expected a valid file path as a single-quoted string; found: {}", arg,
            ),
        }
    }
}

impl ApolloTableFunctions {
    // list sql names of all table functions
    pub(crate) fn keywords() -> &'static [&'static str] {
        &[
            #[cfg(feature = "csv")]
            "read_csv",
            #[cfg(feature = "parquet")]
            "read_parquet",
            #[cfg(feature = "ipc")]
            "read_ipc",
            #[cfg(feature = "json")]
            "read_json",
        ]
    }
}
