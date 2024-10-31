use crate::token::{token::Token, token_literal::TokenLiteral};

#[derive(Debug, PartialEq)]
pub enum AstNode<'a> {
    Object(&'a Token, Vec<AstNode<'a>>, &'a Token),
    Member(&'a Token, &'a Token, &'a AstNode<'a>),
    Array(&'a Token, Vec<AstNode<'a>>, &'a Token),
    Value(&'a TokenLiteral),
}

#[cfg(test)]
mod ast_node_tests {
    use crate::{
        ast_nodes::ast_nodes::AstNode,
        scanner::scanner::Scanner,
        token::{
            token::Token, token_literal::TokenLiteral, token_position::TokenPosition,
            token_type::TokenType,
        },
    };

    #[test]
    fn create_object() {
        let mut scanner = Scanner::new("{ \"m\": false }");
        let res = scanner.scan().unwrap();

        assert_eq!(
            AstNode::Object(
                &Token::new(
                    TokenType::LeftBrace,
                    TokenLiteral::String("{".to_string()),
                    TokenPosition::new(1, 1, 2)
                ),
                vec![AstNode::Member(
                    &Token::new(
                        TokenType::String,
                        TokenLiteral::String("m".to_string()),
                        TokenPosition::new(1, 3, 6)
                    ),
                    &Token::new(
                        TokenType::Colon,
                        TokenLiteral::String(":".to_string()),
                        TokenPosition::new(1, 6, 7)
                    ),
                    &AstNode::Value(&TokenLiteral::Bool(false)),
                )],
                &Token::new(
                    TokenType::RightBrace,
                    TokenLiteral::String("}".to_string()),
                    TokenPosition::new(1, 14, 15)
                ),
            ),
            AstNode::Object(
                &res.get(0).unwrap(),
                vec![AstNode::Member(
                    &res.get(1).unwrap(),
                    &res.get(2).unwrap(),
                    &AstNode::Value(&res.get(3).unwrap().token_literal)
                )],
                &res.get(4).unwrap(),
            )
        );
    }

    #[test]
    fn create_property() {
        let mut scanner = Scanner::new("\"m\": true");
        let res = scanner.scan().unwrap();

        assert_eq!(
            AstNode::Member(
                &Token::new(
                    TokenType::String,
                    TokenLiteral::String("m".to_string()),
                    TokenPosition::new(1, 1, 4)
                ),
                &Token::new(
                    TokenType::Colon,
                    TokenLiteral::String(":".to_string()),
                    TokenPosition::new(1, 4, 5)
                ),
                &AstNode::Value(&TokenLiteral::Bool(true)),
            ),
            AstNode::Member(
                &res.get(0).unwrap(),
                &res.get(1).unwrap(),
                &AstNode::Value(&res.get(2).unwrap().token_literal),
            )
        );
    }

    #[test]
    fn create_array() {
        let mut scanner = Scanner::new("[ true, false ]");
        let res = scanner.scan().unwrap();

        assert_eq!(
            AstNode::Array(
                &Token::new(
                    TokenType::LeftBracket,
                    TokenLiteral::String("[".to_string()),
                    TokenPosition::new(1, 1, 2)
                ),
                vec![
                    AstNode::Value(&TokenLiteral::Bool(true)),
                    AstNode::Value(&TokenLiteral::Bool(false)),
                ],
                &Token::new(
                    TokenType::RightBracket,
                    TokenLiteral::String("]".to_string()),
                    TokenPosition::new(1, 15, 16)
                ),
            ),
            AstNode::Array(
                &res.get(0).unwrap(),
                vec![
                    AstNode::Value(&res.get(1).unwrap().token_literal),
                    AstNode::Value(&res.get(3).unwrap().token_literal)
                ],
                &res.get(4).unwrap()
            )
        );
    }

    #[test]
    fn create_values() {
        let mut scanner = Scanner::new("null");
        let res = scanner.scan().unwrap();
        assert_eq!(
            AstNode::Value(&TokenLiteral::Null),
            AstNode::Value(&res.get(0).unwrap().token_literal)
        );

        let mut scanner = Scanner::new("true");
        let res = scanner.scan().unwrap();
        assert_eq!(
            AstNode::Value(&TokenLiteral::Bool(true)),
            AstNode::Value(&res.get(0).unwrap().token_literal)
        );

        let mut scanner = Scanner::new("false");
        let res = scanner.scan().unwrap();
        assert_eq!(
            AstNode::Value(&TokenLiteral::Bool(false)),
            AstNode::Value(&res.get(0).unwrap().token_literal)
        );

        let mut scanner = Scanner::new("\"hello\"");
        let res = scanner.scan().unwrap();
        assert_eq!(
            AstNode::Value(&TokenLiteral::String("hello".to_string())),
            AstNode::Value(&res.get(0).unwrap().token_literal)
        );

        let mut scanner = Scanner::new("888.888");
        let res = scanner.scan().unwrap();
        assert_eq!(
            AstNode::Value(&TokenLiteral::Number(888.888)),
            AstNode::Value(&res.get(0).unwrap().token_literal)
        );
    }
}
