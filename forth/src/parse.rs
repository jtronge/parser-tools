pub enum ParseError {}

type ParseResult<T> = std::result::Result<T, ParseError>;

pub enum Token {}

pub struct TokenStream;

impl Iterator for TokenStream {
    type Item = ParseResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
