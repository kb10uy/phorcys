//! Error types.

use std::result::Result as StdResult;

use thiserror::Error as ThisError;

/// Represents an error.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ThisError)]
pub enum Error {
    /// Invalid address form.
    #[error("Invalid address")]
    InvalidAddress,

    /// Given bytes is not aligned.
    #[error("Bytes array not aligned")]
    UnalignedData,

    /// Data termination not detected.
    #[error("String termination not found")]
    NotTerminated,

    /// Invalid tag form.
    #[error("Invalid tag")]
    InvalidTag,
}

/// Result type shorthand with `crate::Error`.
pub type Result<T> = StdResult<T, Error>;
