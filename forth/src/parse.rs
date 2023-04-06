//! Forth parsing module
use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum ParseError {}

type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub enum Number {
    Real(i64, u64),
    Integer(i64),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Number(Number),
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
        } else if let Some((num, rest)) = lex_number(self.code.clone()) {
            self.code = rest;
            Some(Ok(Token::Number(num)))
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
pub fn lex_number(s: Peekable<Chars>) -> Option<(Number, Peekable<Chars>)> {
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_integer() {
        let mut stream = TokenStream::new("123");

        assert_eq!(stream.next().unwrap(), Ok(Token::Number(Number::Integer(123))));
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn test_ident() {
        let mut stream = TokenStream::new("test-name");

        assert_eq!(stream.next().unwrap(), Ok(Token::Ident("test-name".into())));
        assert_eq!(stream.next(), None);
    }
}
