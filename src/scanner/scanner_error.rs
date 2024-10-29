use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum ScannerError {
    UnknownCharacter,
    UnterminatedString,
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownCharacter => write!(f, "Unknown character"),
            Self::UnterminatedString => write!(f, "Unterminated string"),
        }
    }
}

impl Error for ScannerError {}

#[cfg(test)]
mod scanner_error {
    use super::ScannerError;

    #[test]
    fn unknown_character() {
        let unknown: Result<&str, ScannerError> = Err(ScannerError::UnknownCharacter);

        assert_eq!(Err(ScannerError::UnknownCharacter), unknown);
        assert_eq!("Unknown character", unknown.unwrap_err().to_string());
    }

    #[test]
    fn unterminated_string() {
        let unterminated: Result<&str, ScannerError> = Err(ScannerError::UnterminatedString);

        assert_eq!(Err(ScannerError::UnterminatedString), unterminated);
        assert_eq!("Unknown character", unterminated.unwrap_err().to_string());
    }
}
