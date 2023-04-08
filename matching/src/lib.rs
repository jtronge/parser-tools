use nom::{
    IResult,
    InputLength,
    Parser,
    Err,
    error::ParseError,
    combinator::{
        recognize,
    },
    sequence::pair,
    branch::alt,
    character::complete::{
        alphanumeric1,
        alpha1,
        digit1,
        char,
    },
    bytes::complete::tag,
    multi::many0_count,
};

/// Similar to many0_count, but discarding the count to keep the return value
/// IResult<I, I, E>.
fn many0_nocount<I, O, E, F>(mut f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: Clone + InputLength,
    O: Default,
    F: Parser<I, O, E>,
    E: ParseError<I>,
{
    move |i| {
        let mut input = i;
        loop {
            let input2 = input.clone();
            match f.parse(input2) {
                // NOTE: No infinite loop check
                Ok((i, _)) => input = i,
                Err(Err::Error(_)) => return Ok((input, O::default())),
                Err(e) => return Err(e),
            }
        }
    }
}

/// Match a C identifier.
pub fn cident(i: &str) -> IResult<&str, &str> {
    recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0_nocount(alt((alphanumeric1, tag("_")))),
        )
    )(i)
}

/// Match a kebab-style identifier.
pub fn kebab_ident(i: &str) -> IResult<&str, &str> {
    recognize(
        pair(
            alt((alpha1, tag("-"))),
            many0_nocount(alt((alphanumeric1, tag("-")))),
        )
    )(i)
}

/// Match a generic number.
pub fn number(i: &str) -> IResult<&str, &str> {
    digit1(i)
}

/// Match a left parenthesis.
pub fn lparen(i: &str) -> IResult<&str, &str> {
    tag("(")(i)
}

/// Match a right parenthesis.
pub fn rparen(i: &str) -> IResult<&str, &str> {
    tag(")")(i)
}

/// Match a comma.
pub fn comma(i: &str) -> IResult<&str, &str> {
    tag(",")(i)
}

/// Match a c string literal.
pub fn cstring_lit(i: &str) -> IResult<&str, &str> {
    // TODO
    alphanumeric1(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        assert!(cident("").is_err());
    }

    #[test]
    fn underscores() {
        assert_eq!(cident("____").unwrap(), ("", "____"));
    }

    #[test]
    fn numbers() {
        assert!(cident("123").is_err());
    }

    #[test]
    fn alpha_num() {
        assert_eq!(cident("a123").unwrap(), ("", "a123"));
    }

    #[test]
    fn underscore_num() {
        assert_eq!(cident("_123").unwrap(), ("", "_123"));
    }

    #[test]
    fn alpha_underscore() {
        assert_eq!(cident("ab_cd_ef_gh_").unwrap(), ("", "ab_cd_ef_gh_"));
    }

    #[test]
    fn empty_cstring_lit() {
        assert_eq!(cstring_lit("\"\"").unwrap(), ("", ""));
    }

    #[test]
    fn nonempty_cstring_lit() {
        assert_eq!(cstring_lit("\"123\"").unwrap(), ("", "123"));
    }

    #[test]
    fn escapes_cstring_lit() {
        assert_eq!(cstring_lit("\"\\r\\n\\t\"").unwrap(), ("", "\r\n\t"));
    }

    #[test]
    fn empty_kebab_ident() {
        assert!(kebab_ident("").is_err());
    }

    #[test]
    fn one_char_kebab_ident() {
        assert_eq!(kebab_ident("a").unwrap(), ("", "a"));
    }

    #[test]
    fn ident_w_dash_kebab_ident() {
        assert_eq!(kebab_ident("abc-def").unwrap(), ("", "abc-def"));
    }

    #[test]
    fn number_empty() {
        assert!(number("").is_err());
    }

    #[test]
    fn number_integer() {
        assert_eq!(number("123").unwrap(), ("", "123"));
    }

    #[test]
    fn number_real() {
        assert_eq!(number("123.89").unwrap(), ("", "123.89"));
    }
}
