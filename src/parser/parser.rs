use crate::{
    ast_nodes::ast_nodes::AstNode,
    token::{token::Token, token_type::TokenType},
};

use super::parser_error::ParserError;

#[derive(Debug)]
pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl<'a> Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    fn parse_value(&'a mut self) -> Result<AstNode<'a>, ParserError> {
        self.next()
            .ok_or(ParserError::UnknownValue)
            .and_then(|token| match token.token_type {
                TokenType::String
                | TokenType::Number
                | TokenType::False
                | TokenType::True
                | TokenType::Null => {
                    return Ok(AstNode::Value(&token.token_literal));
                }
                _ => Err(ParserError::UnknownValue),
            })
    }

    fn next(&mut self) -> Option<&Token> {
        let next = self.tokens.get(self.current);
        self.current += 1;
        next
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::{
        ast_nodes::ast_nodes::AstNode,
        parser::parser_error::ParserError,
        token::{
            token::Token, token_literal::TokenLiteral, token_position::TokenPosition,
            token_type::TokenType,
        },
    };

    use super::Parser;

    #[test]
    fn get_value() {
        let mut parser = Parser::new(vec![Token::new(
            TokenType::True,
            TokenLiteral::Bool(true),
            TokenPosition::new(1, 1, 2),
        )]);
        assert_eq!(
            AstNode::Value(&TokenLiteral::Bool(true)),
            parser.parse_value().unwrap()
        );

        let mut parser = Parser::new(vec![Token::new(
            TokenType::String,
            TokenLiteral::String("hello".to_string()),
            TokenPosition::new(1, 1, 2),
        )]);
        assert_eq!(
            AstNode::Value(&TokenLiteral::String("hello".to_string())),
            parser.parse_value().unwrap()
        );

        let mut parser = Parser::new(vec![Token::new(
            TokenType::Colon,
            TokenLiteral::String(":".to_string()),
            TokenPosition::new(1, 1, 2),
        )]);
        assert_eq!(Err(ParserError::UnknownValue), parser.parse_value(),);
    }

    #[test]
    fn get_current_token() {
        let mut parser = Parser::new(vec![Token::new(
            TokenType::True,
            TokenLiteral::Bool(true),
            TokenPosition::new(1, 1, 2),
        )]);
        let token = parser.next().unwrap();

        assert_eq!(
            Token::new(
                TokenType::True,
                TokenLiteral::Bool(true),
                TokenPosition::new(1, 1, 2),
            ),
            *token
        );
        assert_eq!(1, parser.current);
    }

    #[test]
    fn look_at_current_token() {
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
