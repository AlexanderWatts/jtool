use super::{token_literal::TokenLiteral, token_position::TokenPosition, token_type::TokenType};

///Encapsulate scanned words
///
///A token is a wrapper over some scanned word as a result of lexical analysis providing
///information such as its type, literal value and position.
///
///# Examples
///
///```
///let t1 = Token::new(
///    TokenType::String,
///    TokenLiteral::String("Hello, World!".to_string()),
///    TokenPosition::new(1, 1, 15),
///);
///
///let t2 = Token::new(
///    TokenType::String,
///    TokenLiteral::Null,
///    TokenPosition::new(1, 1, 4),
///);
///```
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub token_literal: TokenLiteral,
    pub token_position: TokenPosition,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        token_literal: TokenLiteral,
        token_position: TokenPosition,
    ) -> Self {
        Self {
            token_type,
            token_literal,
            token_position,
        }
    }
}

#[cfg(test)]
mod token_tests {
    use crate::token::{
        token_literal::TokenLiteral, token_position::TokenPosition, token_type::TokenType,
    };

    use super::Token;

    #[test]
    fn create_new_token() {
        let t1 = Token::new(
            TokenType::String,
            TokenLiteral::String("Hello, World!".to_string()),
            TokenPosition::new(1, 1, 15),
        );

        assert_eq!(TokenType::String, t1.token_type);
        assert_eq!(
            TokenLiteral::String("Hello, World!".to_string()),
            t1.token_literal
        );
        assert_eq!(TokenPosition::new(1, 1, 15), t1.token_position)
    }
}
