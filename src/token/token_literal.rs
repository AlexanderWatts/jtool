#[derive(Debug)]
pub enum TokenLiteral {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

impl From<TokenLiteral> for String {
    fn from(token_literal: TokenLiteral) -> Self {
        match token_literal {
            TokenLiteral::String(value) => value.to_string(),
            TokenLiteral::Number(value) => value.to_string(),
            TokenLiteral::Bool(value) => value.to_string(),
            TokenLiteral::Null => "null".to_string(),
        }
    }
}

#[cfg(test)]
mod token_literal_tests {
    use super::TokenLiteral;

    #[test]
    fn token_literals_into_string() {
        let string_literal: String = TokenLiteral::String("Hello, World!".to_string()).into();
        let number_literal: String = TokenLiteral::Number(100.0).into();
        let float_literal: String = TokenLiteral::Number(123.456).into();
        let false_literal: String = TokenLiteral::Bool(false).into();
        let true_literal: String = TokenLiteral::Bool(true).into();
        let null_literal: String = TokenLiteral::Null.into();

        assert_eq!(String::from("Hello, World!"), string_literal);
        assert_eq!(String::from("100"), number_literal);
        assert_eq!(String::from("123.456"), float_literal);
        assert_eq!(String::from("null"), null_literal);
        assert_eq!(String::from("false"), false_literal);
        assert_eq!(String::from("true"), true_literal);
    }
}
