use super::{token_literal::TokenLiteral, token_type::TokenType};

pub struct Token {
    token_type: TokenType,
    token_literal: TokenLiteral,
}

impl Token {
    pub fn new(token_type: TokenType, token_literal: TokenLiteral) -> Self {
        Self {
            token_type,
            token_literal,
        }
    }
}

#[cfg(test)]
mod token_tests {
    use crate::token::{token_literal::TokenLiteral, token_type::TokenType};

    use super::Token;

    #[test]
    fn create_new_token() {
        let t1 = Token::new(
            TokenType::String,
            TokenLiteral::String("Hello, World!".to_string()),
        );

        assert_eq!(TokenType::String, t1.token_type);
        assert_eq!(
            TokenLiteral::String("Hello, World!".to_string()),
            t1.token_literal
        );
    }
}
