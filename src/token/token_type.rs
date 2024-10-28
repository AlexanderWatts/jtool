#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    True,
    False,
    Null,
    String,
    Number,
    Identifier,
    Eof,
}

#[cfg(test)]
mod token_type_tests {
    use crate::token::token_type::TokenType;
   
    #[test]
    fn token_type_comparison() {
        assert_eq!(TokenType::LeftBrace, TokenType::LeftBrace);
        assert_ne!(TokenType::Identifier, TokenType::String);
    }
}
