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
        ast_nodes::nodes::AstNode, scanner::scanner::Scanner, token::token_literal::TokenLiteral,
    };

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
