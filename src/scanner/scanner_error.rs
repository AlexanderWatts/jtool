use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum ScannerError {
    UnknownCharacter,
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownCharacter => write!(f, "Unknown character"),
        }
    }
}

impl Error for ScannerError {}

#[cfg(test)]
mod scanner_error {
    use super::ScannerError;

    #[test]
    fn scanner_error() {
        let unknown: Result<&str, ScannerError> = Err(ScannerError::UnknownCharacter);

        assert_eq!(Err(ScannerError::UnknownCharacter), unknown);
        assert_eq!("Unknown character", unknown.unwrap_err().to_string());
    }
}
