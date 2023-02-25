//! Tokenization code.
use nom::{
    error::VerboseError,
    character::complete::{
        space0,
        alphanumeric1,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub enum TokenError {}

type Result<T> = std::result::Result<T, TokenError>;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// C Identifier
    Ident(String),
    /// String literal
    StringLit(String),
    /// ,
    Comma,
    /// (
    LParen,
    /// )
    RParen,
}

/// To be used for iterating over the tokens of an &str.
pub struct TokenIter<'a> {
    input: &'a str,
}

impl<'a> TokenIter<'a> {
    pub fn new(input: &'a str) -> TokenIter<'a> {
        TokenIter {
            input,
        }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let (i, _) = space0::<&str, VerboseError<&str>>(self.input).ok()?;
        match alphanumeric1::<&str, VerboseError<&str>>(i) {
            Ok((i, ident)) => {
                self.input = i;
                Some(Ok(Token::Ident(ident.to_string())))
            }
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_ident() {
        let mut it = TokenIter::new(" abc ");
        assert_eq!(it.next(), Some(Ok(Token::Ident("abc".to_string()))));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn two_ident() {
        let mut it = TokenIter::new(" abc def ");
        assert_eq!(it.next(), Some(Ok(Token::Ident("abc".to_string()))));
        assert_eq!(it.next(), Some(Ok(Token::Ident("def".to_string()))));
        assert_eq!(it.next(), None);
    }
}
