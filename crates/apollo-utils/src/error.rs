use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use crate::config::verbose;
use crate::format_pl_smallstr;

type ErrString = Cow<'static, str>;

#[derive(Debug)]
pub enum ApolloUtilsError {
    ComputeError(ErrString),
}

impl Display for ApolloUtilsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApolloUtilsError::ComputeError(s) => {
                let s = s.as_ref();
                write!(f, "{s}")
            },
        }
    }
}

pub type Result<T> = std::result::Result<T, ApolloUtilsError>;

/// Utility whose Display impl truncates the string unless APOLLO_VERBOSE is set.
pub struct TruncateErrorDetail<'a>(pub &'a str);

impl std::fmt::Display for TruncateErrorDetail<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let maybe_truncated = if verbose() {
            self.0
        } else {
            // Clamp the output on non-verbose
            &self.0[..self.0.len().min(4096)]
        };

        f.write_str(maybe_truncated)?;

        if maybe_truncated.len() != self.0.len() {
            let n_more = self.0.len() - maybe_truncated.len();
            f.write_str(" ...(set APOLLO_VERBOSE=1 to see full response (")?;
            f.write_str(&format_pl_smallstr!("{}", n_more))?;
            f.write_str(" more characters))")?;
        };

        Ok(())
    }
}
