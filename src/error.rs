
use std::fmt::{Display, Formatter};

/// Common error type for the fppa-rs library.
#[derive(Debug)]
pub enum FppaError {
    /// An input value or argument is invalid.
    InvalidInput(String),

    /// An operation is invalid in the current state.
    InvalidState(String),

    /// The requested operation is not supported.
    UnsupportedOperation(String),

    /// An operating-system or standard-library I/O error.
    Io(std::io::Error),
}

/// Convenient result type for fppa-rs operations.
pub type FppaResult<T> = Result<T, FppaError>;

impl Display for FppaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(message) => {
                write!(f, "Invalid input: {message}")
            }
            Self::InvalidState(message) => {
                write!(f, "Invalid state: {message}")
            }
            Self::UnsupportedOperation(message) => {
                write!(f, "Unsupported operation: {message}")
            }
            Self::Io(error) => {
                write!(f, "I/O error: {error}")
            }
        }
    }
}

impl std::error::Error for FppaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for FppaError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
