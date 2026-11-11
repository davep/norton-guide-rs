//! Provides the error types for the Norton Guide library.

use thiserror::Error;

/// Error type for the Norton Guide library.
#[derive(Error, Debug)]
pub enum NortonGuideError {
    /// Error variant for an unexpected magic number in the Norton Guide file.
    #[error("unexpected magic number {found:?}")]
    BadMagicError { found: u16 },

    #[error("unexpected entry type {found:?}")]
    UnexpectedEntryTypeError { found: u16 },

    /// Error variant for an unexpected entry type in the Norton Guide file.
    #[error("unexpected entry type: expected {expected}, found {found}")]
    BadEntryTypeError { expected: u16, found: u16 },

    #[error("entry is not a short entry")]
    NotShortError,

    #[error("entry is not a long entry")]
    NotLongError,

    /// Error variant for IO errors.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

// errors.rs ends here
