use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ParseError {
    HeaderSizeMismatch,
    Io(io::Error),
    Other,
}

const SIZE_MISMATCH_MESSAGE: &str = "Failed to parse AmxModX file, header size mismatch";
const OTHER_ERROR_MESSAGE: &str = "Failed to parse AmxModX file";

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::HeaderSizeMismatch => write!(f, "{}", SIZE_MISMATCH_MESSAGE),
            ParseError::Io(ref io_err) => {
                write!(f, "Failed to parse AmxModX file, IO error: {}", io_err)
            }
            ParseError::Other => write!(f, "{}", OTHER_ERROR_MESSAGE),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match self {
            ParseError::HeaderSizeMismatch => SIZE_MISMATCH_MESSAGE,
            ParseError::Io(_) => "Failed to parse AmxModX file, IO error",
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

    use super::{ParseError, OTHER_ERROR_MESSAGE, SIZE_MISMATCH_MESSAGE};

    fn io_error() -> io::Error {
        io::Error::new(io::ErrorKind::Other, "oops!")
    }

    // HACK :))
    fn parse_error_variants() -> Vec<ParseError> {
        vec![
            ParseError::HeaderSizeMismatch,
            ParseError::Io(io_error()),
            ParseError::Other,
        ]
    }

    #[test]
    fn test_display() {
        for variant in parse_error_variants().into_iter() {
            match variant {
                ParseError::HeaderSizeMismatch => assert_eq!(
                    SIZE_MISMATCH_MESSAGE.to_owned(),
                    format!("{}", ParseError::HeaderSizeMismatch)
                ),
                ParseError::Io(_) => assert_eq!(
                    "Failed to parse AmxModX file, IO error: oops!".to_owned(),
                    format!("{}", ParseError::Io(io_error()))
                ),
                ParseError::Other => {
                    assert_eq!(OTHER_ERROR_MESSAGE, format!("{}", ParseError::Other))
                }
            }
        }
    }

    #[test]
    fn test_description() {
        for variant in parse_error_variants().into_iter() {
            match variant {
                ParseError::HeaderSizeMismatch => assert_eq!(
                    SIZE_MISMATCH_MESSAGE.to_owned(),
                    ParseError::HeaderSizeMismatch.description()
                ),
                ParseError::Io(_) => assert_eq!(
                    "Failed to parse AmxModX file, IO error".to_owned(),
                    ParseError::Io(io_error()).description()
                ),
                ParseError::Other => {
                    assert_eq!(OTHER_ERROR_MESSAGE, ParseError::Other.description())
                }
            }
        }
    }

    #[test]
    fn test_convert_into_io_error() {
        for variant in parse_error_variants().into_iter() {
            let expected_formatting = match variant {
                // Should save original io object, not creating new message
                ParseError::Io(ref io_err) => io_error().description().to_owned(),
                _ => format!("{}", variant),
            };

            let io_error: io::Error = variant.into();
            assert_eq!(io_error.kind(), io::ErrorKind::Other);
            assert_eq!(io_error.description(), expected_formatting);
        }
    }
}
