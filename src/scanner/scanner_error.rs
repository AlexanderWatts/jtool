use std::{error::Error, fmt::Display};

use crate::token::token_position::TokenPosition;

#[derive(Debug, PartialEq)]
pub enum ScannerError {
    UnknownCharacter(TokenPosition, char),
    UnknownLiteral(TokenPosition, String),
    UnterminatedString(TokenPosition),
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownCharacter(
                TokenPosition {
                    line,
                    column_start,
                    column_end,
                    ..
                },
                character,
            ) => write!(
                f,
                "Error at [line:{}, between:{}-{}] Unknown character [{}]",
                line, column_start, column_end, character,
            ),
            Self::UnknownLiteral(
                TokenPosition {
                    line,
                    column_start,
                    column_end,
                    ..
                },
                content,
            ) => write!(
                f,
                "Error at [line:{}, between:{}-{}] Unknown literal [{}]",
                line, column_start, column_end, content,
            ),
            Self::UnterminatedString(TokenPosition {
                line,
                column_start,
                column_end,
                ..
            }) => write!(
                f,
                "Error at [line:{}, between:{}-{}] Unterminated string",
                line, column_start, column_end,
            ),
        }
    }
}

impl Error for ScannerError {}

#[cfg(test)]
mod scanner_error {
    use crate::token::token_position::TokenPosition;

    use super::ScannerError;

    #[test]
    fn unknown_character() {
        let unknown: Result<&str, ScannerError> = Err(ScannerError::UnknownCharacter(
            TokenPosition::new(1, 1, 2),
            '@',
        ));

        assert_eq!(
            "Error at [line:1, between:1-2] Unknown character [@]",
            unknown.unwrap_err().to_string()
        );
    }

    #[test]
    fn unknown_literal() {
        let unknown_literal: Result<&str, ScannerError> = Err(ScannerError::UnknownLiteral(
            TokenPosition::new(1, 1, 6),
            "hello".to_string(),
        ));

        assert_eq!(
            "Error at [line:1, between:1-6] Unknown literal [hello]",
            unknown_literal.unwrap_err().to_string()
        );
    }

    #[test]
    fn unterminated_string() {
        let unterminated: Result<&str, ScannerError> = Err(ScannerError::UnterminatedString(
            TokenPosition::new(1, 1, 6),
        ));

        assert_eq!(
            "Error at [line:1, between:1-6] Unterminated string",
            unterminated.unwrap_err().to_string()
        );
    }
}
