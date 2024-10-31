use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnknownValue,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownValue => write!(f, ""),
        }
    }
}

impl Error for ParserError {}
