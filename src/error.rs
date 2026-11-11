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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_variants_have_readable_messages() {
        let invalid_input = FppaError::InvalidInput("buffer size".to_string());

        let invalid_state = FppaError::InvalidState("device not initialised".to_string());

        let unsupported = FppaError::UnsupportedOperation("DMA transfer".to_string());

        assert_eq!(invalid_input.to_string(), "Invalid input: buffer size");

        assert_eq!(
            invalid_state.to_string(),
            "Invalid state: device not initialised"
        );

        assert_eq!(
            unsupported.to_string(),
            "Unsupported operation: DMA transfer"
        );
    }

    #[test]
    fn io_error_conversion_preserves_original_error() {
        let original = std::io::Error::new(std::io::ErrorKind::Other, "device unavailable");

        let error = FppaError::from(original);

        assert_eq!(error.to_string(), "I/O error: device unavailable");

        let source =
            std::error::Error::source(&error).expect("the original I/O error should be preserved");

        assert_eq!(source.to_string(), "device unavailable");
    }
}
