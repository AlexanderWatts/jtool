use super::{token_literal::TokenLiteral, token_position::TokenPosition, token_type::TokenType};

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    token_literal: TokenLiteral,
    token_position: TokenPosition,
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
            TokenPosition::new(1, 1, 1),
        );

        assert_eq!(TokenType::String, t1.token_type);
        assert_eq!(
            TokenLiteral::String("Hello, World!".to_string()),
            t1.token_literal
        );
        assert_eq!(TokenPosition::new(1, 1, 1), t1.token_position)
    }
}
