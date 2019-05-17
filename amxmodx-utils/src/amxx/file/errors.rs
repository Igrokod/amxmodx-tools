use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ParseError {
    HeaderSizeMismatch,
    MagicMismatch,
    // Supported / Requested
    UnsupportedVersion { supported: u16, requested: u16 },
    NoSections,
    Io(io::Error),
    InvalidSection,
    Other,
}

const SIZE_MISMATCH_MESSAGE: &str = "Header size mismatch";
const MAGIC_MISMATCH_MESSAGE: &str = "invalid file magic";
const UNSUPPORTED_VERSION_MESSAGE: &str = "Unsupported file version";
const NO_SECTIONS_MESSAGE: &str = "File got no sections to analyze";
const IO_ERROR_MESSAGE: &str = "Failed to parse AmxModX file, IO error";
const INVALID_SECTION_MESSAGE: &str = "Failed to parse AmxModX section";
const OTHER_ERROR_MESSAGE: &str = "Failed to parse AmxModX file";

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::HeaderSizeMismatch => write!(f, "{}", SIZE_MISMATCH_MESSAGE),
            ParseError::MagicMismatch => write!(f, "{}", MAGIC_MISMATCH_MESSAGE),
            ParseError::UnsupportedVersion {
                supported,
                requested,
            } => write!(
                f,
                "{}, supported: {}, requested: {}",
                UNSUPPORTED_VERSION_MESSAGE, supported, requested
            ),
            ParseError::NoSections => write!(f, "{}", NO_SECTIONS_MESSAGE),
            ParseError::Io(ref io_err) => write!(f, "{}: {}", IO_ERROR_MESSAGE, io_err),
            ParseError::InvalidSection => write!(f, "{}", INVALID_SECTION_MESSAGE),
            ParseError::Other => write!(f, "{}", OTHER_ERROR_MESSAGE),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match self {
            ParseError::HeaderSizeMismatch => SIZE_MISMATCH_MESSAGE,
            ParseError::MagicMismatch => MAGIC_MISMATCH_MESSAGE,
            ParseError::UnsupportedVersion {
                supported: _,
                requested: _,
            } => UNSUPPORTED_VERSION_MESSAGE,
            ParseError::NoSections => NO_SECTIONS_MESSAGE,
            ParseError::Io(_) => IO_ERROR_MESSAGE,
            ParseError::InvalidSection => INVALID_SECTION_MESSAGE,
            ParseError::Other => OTHER_ERROR_MESSAGE,
        }
    }
}

impl Into<io::Error> for ParseError {
    fn into(self) -> io::Error {
        match self {
            ParseError::Io(io_err) => io_err,
            _ => io::Error::new(io::ErrorKind::Other, format!("{}", self)),
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(io_error: io::Error) -> ParseError {
        ParseError::Io(io_error)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::io;

    use super::{
        ParseError, INVALID_SECTION_MESSAGE, IO_ERROR_MESSAGE, MAGIC_MISMATCH_MESSAGE,
        NO_SECTIONS_MESSAGE, OTHER_ERROR_MESSAGE, SIZE_MISMATCH_MESSAGE,
        UNSUPPORTED_VERSION_MESSAGE,
    };

    fn io_error() -> io::Error {
        io::Error::new(io::ErrorKind::Other, "oops!")
    }

    // HACK :))
    fn parse_error_variants() -> Vec<ParseError> {
        vec![
            ParseError::HeaderSizeMismatch,
            ParseError::MagicMismatch,
            ParseError::UnsupportedVersion {
                supported: 1,
                requested: 2,
            },
            ParseError::NoSections,
            ParseError::Io(io_error()),
            ParseError::InvalidSection,
            ParseError::Other,
        ]
    }

    #[test]
    fn test_display() {
        for variant in parse_error_variants().into_iter() {
            let expected_message = match variant {
                ParseError::HeaderSizeMismatch => SIZE_MISMATCH_MESSAGE,
                ParseError::MagicMismatch => MAGIC_MISMATCH_MESSAGE,
                ParseError::UnsupportedVersion {
                    supported: _,
                    requested: _,
                } => "Unsupported file version, supported: 1, requested: 2",
                ParseError::NoSections => NO_SECTIONS_MESSAGE,
                ParseError::Io(_) => "Failed to parse AmxModX file, IO error: oops!",
                ParseError::InvalidSection => INVALID_SECTION_MESSAGE,
                ParseError::Other => OTHER_ERROR_MESSAGE,
            };

            assert_eq!(expected_message, format!("{}", variant))
        }
    }

    #[test]
    fn test_description() {
        for variant in parse_error_variants().into_iter() {
            let expected_message = match variant {
                ParseError::HeaderSizeMismatch => SIZE_MISMATCH_MESSAGE,
                ParseError::MagicMismatch => MAGIC_MISMATCH_MESSAGE,
                ParseError::UnsupportedVersion {
                    supported: _,
                    requested: _,
                } => UNSUPPORTED_VERSION_MESSAGE,
                ParseError::NoSections => NO_SECTIONS_MESSAGE,
                ParseError::Io(_) => IO_ERROR_MESSAGE,
                ParseError::InvalidSection => INVALID_SECTION_MESSAGE,
                ParseError::Other => OTHER_ERROR_MESSAGE,
            };

            assert_eq!(expected_message, variant.description());
        }
    }

    #[test]
    fn test_convert_into_io_error() {
        for variant in parse_error_variants().into_iter() {
            let expected_formatting = match variant {
                // Should save original io object, not creating new message
                ParseError::Io(_) => io_error().description().to_owned(),
                _ => format!("{}", variant),
            };

            let io_error: io::Error = variant.into();
            assert_eq!(io_error.kind(), io::ErrorKind::Other);
            assert_eq!(io_error.description(), expected_formatting);
        }
    }
}
