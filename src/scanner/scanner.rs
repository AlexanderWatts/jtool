pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: String::from(source),
            start: 0,
            current: 0,
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
    fn create_new_scanner() {
        let scanner = Scanner::new("true");

        assert_eq!("true", scanner.source);
        assert_eq!(0, scanner.start);
        assert_eq!(0, scanner.current);
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
}
