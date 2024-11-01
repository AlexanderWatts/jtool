use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedToken(String),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(error_message) => write!(f, "{}", error_message),
        }
    }
}

impl Error for ParserError {}
