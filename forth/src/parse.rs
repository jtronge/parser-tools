//! Forth parsing module
use std::str::Chars;
use std::iter::Peekable;
use matching::{kebab_ident, number, space};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidToken,
}

type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Number(String),
}

pub struct TokenStream<'a> {
    // code: Peekable<Chars<'a>>,
    code: &'a str,
}

impl<'a> TokenStream<'a> {
    pub fn new(code: &'a str) -> TokenStream<'a> {
        TokenStream {
            code,
        }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = ParseResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip spaces if necessary
        if let Ok((i, s)) = space(self.code) {
            self.code = i;
        }
        if self.code.len() == 0 {
            return None;
        }
        if let Ok((i, ident)) = kebab_ident(self.code) {
            self.code = i;
            Some(Ok(Token::Ident(ident.into())))
        } else if let Ok((i, num)) = number(self.code) {
            self.code = i;
            Some(Ok(Token::Number(num.into())))
        } else {
            Some(Err(ParseError::InvalidToken))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_integer() {
        let mut stream = TokenStream::new("123");

        assert_eq!(stream.next().unwrap(), Ok(Token::Number("123".into())));
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn test_ident() {
        let mut stream = TokenStream::new("test-name");

        assert_eq!(stream.next().unwrap(), Ok(Token::Ident("test-name".into())));
        assert_eq!(stream.next(), None);
    }
}
