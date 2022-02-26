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

    /// Type tag has unknown type.
    #[error("Unknown data type: {0}")]
    UnknownType(u8),

    /// Shortage in argument data bytes.
    #[error("Not enough argument data")]
    NotEnoughData,

    /// Illegal types tag detected.
    #[error("Type tag structure is illegal")]
    IllegalStructure,

    #[error("Invalid address pattern: {0}")]
    InvalidPattern(String),
}

/// Result type shorthand with `crate::Error`.
pub type Result<T> = StdResult<T, Error>;
