//! Forth parsing module
#[derive(Debug, PartialEq)]
pub enum ParseError {}

type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Integer(i64),
}

pub struct TokenStream<'a> {
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
        None
    }
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
