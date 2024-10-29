use std::usize;

use crate::token::token::Token;

use super::scanner_error::ScannerError;

#[derive(Debug)]
pub struct Scanner {
    pub source: String,
    pub start: usize,
    pub current: usize,
    pub line: i32,
    pub column_start: i32,
    pub column_end: i32,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: String::from(source),
            start: 0,
            current: 0,
            line: 1,
            column_start: 1,
            column_end: 1,
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, ScannerError> {
        let mut tokens: Vec<Token> = vec![];

        while let Some(current_char) = self.next() {
            if let Some(token) = self.eval(current_char)? {
                tokens.push(token);
            };
        }

        Ok(tokens)
    }

    fn eval(&mut self, current_char: char) -> Result<Option<Token>, ScannerError> {
        match current_char {
            '\n' => {
                self.line += 1;
                Ok(None)
            }
            ' ' | '\t' | '\r' => Ok(None),
            _ => Err(ScannerError::UnknownCharacter),
        }
    }

    fn next(&mut self) -> Option<char> {
        let char = self.source.chars().nth(self.current);
        self.current += 1;
        char
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }
}

#[cfg(test)]
mod scanner_tests {
    use super::Scanner;

    #[test]
    fn scan() {
        let mut s1 = Scanner::new("{}");
        let _ = s1.scan();
    }

    #[test]
    fn scan_error() {
        let mut s1 = Scanner::new("@");
        let res = s1.scan();

        assert_eq!("Unknown character", res.unwrap_err().to_string())
    }

    #[test]
    fn eval_new_line() {
        let mut s1 = Scanner::new("\n");
        let _ = s1.eval('\n');

        assert_eq!(2, s1.line)
    }

    #[test]
    fn eval_current_character() {
        let mut s1 = Scanner::new("{}");

        let unknown = s1.eval('@');
        assert_eq!("Unknown character", unknown.unwrap_err().to_string());

        let space = s1.eval(' ').unwrap();
        assert_eq!(None, space);
    }

    #[test]
    fn next() {
        let mut s1 = Scanner::new("{}");

        let first = s1.next();

        assert_eq!(Some('{'), first);
        assert_eq!(1, s1.current);

        let second = s1.next();

        assert_eq!(Some('}'), second);
        assert_eq!(2, s1.current);

        let none = s1.next();
        assert_eq!(None, none);
        assert_eq!(3, s1.current);
    }

    #[test]
    fn next_to_end_of_source() {
        let mut s1 = Scanner::new("true");

        let mut buf = String::new();

        while let Some(current) = s1.next() {
            buf += &current.to_string();
        }

        assert_eq!("true", buf);
        assert_eq!(5, s1.current);
    }

    #[test]
    fn peek() {
        let s1 = Scanner::new("true");

        let peek = s1.peek();

        assert_eq!(Some('t'), peek);
        assert_eq!(0, s1.current);
    }

    #[test]
    fn create_new_scanner() {
        let scanner = Scanner::new("true");

        assert_eq!("true", scanner.source);
        assert_eq!(0, scanner.start);
        assert_eq!(0, scanner.current);
        assert_eq!(1, scanner.line);
        assert_eq!(1, scanner.column_start);
        assert_eq!(1, scanner.column_end);
    }
}
