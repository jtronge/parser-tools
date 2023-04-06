//! Forth parsing module
use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum ParseError {}

type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Integer(i64),
}

pub struct TokenStream<'a> {
    code: Peekable<Chars<'a>>,
}

impl<'a> TokenStream<'a> {
    pub fn new(code: &'a str) -> TokenStream<'a> {
        TokenStream {
            code: code.chars().peekable(),
        }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = ParseResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((ident, rest)) = lex_ident(self.code.clone()) {
            self.code = rest;
            Some(Ok(Token::Ident(ident)))
        } else if let Some((i, rest)) = lex_number(self.code.clone()) {
            self.code = rest;
            Some(Ok(Token::Integer(i)))
        } else {
            None
        }
    }
}

/// Tokenize an identifier
pub fn lex_ident(s: Peekable<Chars>) -> Option<(String, Peekable<Chars>)> {
    None
}

/// Tokenize a number
pub fn lex_number(s: Peekable<Chars>) -> Option<(i64, Peekable<Chars>)> {
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_integer() {
        let mut stream = TokenStream::new("123");

        assert_eq!(stream.next().unwrap(), Ok(Token::Integer(123)));
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn test_ident() {
        let mut stream = TokenStream::new("test-name");

        assert_eq!(stream.next().unwrap(), Ok(Token::Ident("test-name".into())));
        assert_eq!(stream.next(), None);
    }
}
