use std::cell::Cell;

use crate::{
    ast_nodes::ast_nodes::AstNode,
    token::{token::Token, token_type::TokenType},
};

use super::parser_error::ParserError;

#[derive(Debug)]
pub struct Parser {
    current: Cell<usize>,
    tokens: Vec<Token>,
}

impl<'a> Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            current: Cell::new(0),
            tokens,
        }
    }

    pub fn parse(&'a mut self) -> Result<AstNode<'a>, ParserError> {
        if let Some(_) = self.peek() {
            return self.parse_list();
        }

        self.parse_value()
    }

    fn parse_list(&'a self) -> Result<AstNode<'a>, ParserError> {
        let left_brace = self.next_or_error(TokenType::LeftBracket, "Expected [")?;

        let members = vec![];

        let right_brace = self.next_or_error(TokenType::RightBracket, "Expected ]")?;

        Ok(AstNode::Array(left_brace, members, right_brace))
    }

    fn parse_value(&'a mut self) -> Result<AstNode<'a>, ParserError> {
        self.next()
            .ok_or(ParserError::UnexpectedToken("".to_string()))
            .and_then(|token| match token.token_type {
                TokenType::String
                | TokenType::Number
                | TokenType::False
                | TokenType::True
                | TokenType::Null => {
                    return Ok(AstNode::Value(&token.token_literal));
                }
                _ => Err(ParserError::UnexpectedToken("".to_string())),
            })
    }

    fn next_or_error(
        &self,
        token_type: TokenType,
        error_message: &str,
    ) -> Result<&Token, ParserError> {
        if let Some(token) = self.peek() {
            if token.token_type == token_type {
                return Ok(self.next().unwrap());
            }
        }
        Err(ParserError::UnexpectedToken(error_message.to_string()))
    }

    fn next(&self) -> Option<&Token> {
        let next = self.tokens.get(self.current.take());
        self.current.set(self.current.get() + 1);
        next
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current.get())
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::{
        ast_nodes::ast_nodes::AstNode,
        parser::parser_error::ParserError,
        scanner::scanner::Scanner,
        token::{
            token::Token, token_literal::TokenLiteral, token_position::TokenPosition,
            token_type::TokenType,
        },
    };

    use super::Parser;

    #[test]
    fn get_list() {
        let mut scanner = Scanner::new("[true]");
        let tokens = scanner.scan().unwrap();
        let parser = Parser::new(tokens);

        let _ = parser.parse_list();
    }

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
        assert_eq!(
            Err(ParserError::UnexpectedToken("".to_string())),
            parser.parse_value()
        );
    }

    #[test]
    fn get_current_token() {
        let parser = Parser::new(vec![Token::new(
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
        assert_eq!(1, parser.current.get());
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
