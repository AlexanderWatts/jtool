///Track a tokens position within its environment
#[derive(Debug, PartialEq)]
pub struct TokenPosition {
    pub line: i32,
    pub column_start: i32,
    pub column_end: i32,
    pub span: i32,
}

impl TokenPosition {
    pub fn new(line: i32, column_start: i32, column_end: i32) -> Self {
        Self {
            line,
            column_start,
            column_end,
            span: column_end - column_start,
        }
    }
}

#[cfg(test)]
mod token_position_tests {
    use super::TokenPosition;

    #[test]
    fn create_new_token_position() {
        let token_position = TokenPosition::new(1, 1, 4);

        assert_eq!(1, token_position.line);
        assert_eq!(1, token_position.column_start);
        assert_eq!(4, token_position.column_end);
        assert_eq!(3, token_position.span);
    }
}
