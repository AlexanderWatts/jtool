use crate::token::token::Token;

#[derive(Debug)]
pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl<'a> Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::token::{
        token::Token, token_literal::TokenLiteral, token_position::TokenPosition,
        token_type::TokenType,
    };

    use super::Parser;

    #[test]
    fn get_current_token() {
        let parser = Parser::new(vec![Token::new(
            TokenType::True,
            TokenLiteral::Bool(true),
            TokenPosition::new(1, 1, 2),
        )]);
        let token = parser.peek().unwrap();

        assert_eq!(
            Token::new(
                TokenType::True,
                TokenLiteral::Bool(true),
                TokenPosition::new(1, 1, 2),
            ),
            *token
        );
    }
}
