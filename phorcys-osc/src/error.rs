//! Error types.

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
};

use thiserror::Error as ThisError;

/// Represents an error.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ThisError)]
pub enum Error {
    /// Given path string is inalid.
    InvalidPath,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::InvalidPath => write!(f, "Invalid path given"),
        }
    }
}

/// Result type shorthand with `crate::Error`.
pub type Result<T> = StdResult<T, Error>;
