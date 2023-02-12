use std::str::Chars;
use std::iter::Peekable;
use std::path::Path;
use std::char;

#[derive(Debug, PartialEq)]
pub enum ConstantKind {
    Integer,
    Floating,
    Enumeration,
    Character,
}

macro_rules! token_literals {
    { $( #[$top_level_attr:meta] )*  pub enum $name:ident { $(
        #[literal($lit:expr)]
        $option:ident,
    )* } } => (
        $( #[$top_level_attr] )*
        pub enum $name {
            $(
                $option,
             )*
        }

        impl $name {
            /// Lex the literals.
            pub fn lex<'a>(mut cp: Peekable<Chars<'a>>) -> Option<($name, Peekable<Chars<'a>>, usize)> {
                let mut best = None;
                $(
                    let mut i = 0;
                    let lit = $lit;
                    let mut tmpl = lit.chars().peekable();
                    let mut tmpc = cp.clone();
                    while tmpl.peek() == tmpc.peek() {
                        tmpl.next();
                        tmpc.next();
                    }
                    if tmpl.next() == None {
                        if best.is_some() {
                            let (_, _, j) = best.as_ref().unwrap();
                            if i > *j {
                                let _ = best.insert(($name::$option, tmpc, i));
                            }
                        } else {
                            let _ = best.insert(($name::$option, tmpc, i));
                        }
                    }
                )*
                best
            }
        }
    )
}

token_literals!(
    #[derive(Debug, PartialEq)]
    pub enum PunctuatorKind {
        #[literal("[")]
        OpenBracket,
        #[literal("]")]
        CloseBracket,
        #[literal("(")]
        OpenParenthesis,
        #[literal(")")]
        CloseParenthesis,
        #[literal("{")]
        OpenBrace,
        #[literal("}")]
        CloseBrace,
        #[literal(".")]
        Period,
        #[literal("->")]
        Arrow,
        #[literal("++")]
        Increment,
        #[literal("--")]
        Decrement,
        #[literal("&")]
        Ampersand,
        #[literal("*")]
        Star,
        #[literal("+")]
        Plus,
        #[literal("-")]
        Minus,
        #[literal("~")]
        Tilde,
        #[literal("!")]
        Exclamation,

        #[literal("/")]
        ForwardSlash,
        #[literal("%")]
        Percent,
        #[literal("<<")]
        LeftShift,
        #[literal(">>")]
        RightShift,
        #[literal("<")]
        LessThan,
        #[literal(">")]
        GreaterThan,
        #[literal("<=")]
        LessThanEqual,
        #[literal(">=")]
        GreaterThanEqual,
        #[literal("==")]
        DoubleEqual,
        #[literal("!=")]
        NotEqual,
        #[literal("^")]
        Carrot,
        #[literal("|")]
        Pipe,
        #[literal("&&")]
        LogicalAnd,
        #[literal("||")]
        LogicalOr,

        #[literal("?")]
        QuestionMark,
        #[literal(":")]
        Colon,
        #[literal(";")]
        Semicolon,
        #[literal("...")]
        Ellipsis,

        #[literal("=")]
        Equal,
        #[literal("*=")]
        TimesEqual,
        #[literal("/=")]
        DivideEqual,
        #[literal("%=")]
        PercentEqual,
        #[literal("+=")]
        PlusEqual,
        #[literal("-=")]
        MinusEqual,
        #[literal("<<=")]
        LeftShiftEqual,
        #[literal(">>=")]
        RightShiftEqual,
        #[literal("&=")]
        BitAndEqual,
        #[literal("^=")]
        BitXorEqual,
        #[literal("|=")]
        BitOrEqual,

        #[literal(",")]
        Comma,
        #[literal("#")]
        Pound,
        #[literal("##")]
        DoublePound,
    }
);

token_literals!(
    #[derive(Debug, PartialEq)]
    pub enum KeywordKind {
        #[literal("auto")]
        Auto,
        #[literal("break")]
        Break,
        #[literal("case")]
        Case,
        #[literal("char")]
        Char,
        #[literal("const")]
        Const,
        #[literal("continue")]
        Continue,
        #[literal("default")]
        Default,
        #[literal("do")]
        Do,
        #[literal("double")]
        Double,
        #[literal("else")]
        Else,
        #[literal("enum")]
        Enum,
        #[literal("extern")]
        Extern,
        #[literal("float")]
        Float,
        #[literal("for")]
        For,
        #[literal("goto")]
        Goto,
        #[literal("if")]
        If,
        #[literal("inline")]
        Inline,
        #[literal("int")]
        Int,
        #[literal("long")]
        Long,
        #[literal("register")]
        Register,
        #[literal("restrict")]
        Restrict,
        #[literal("return")]
        Return,
        #[literal("short")]
        Short,
        #[literal("signed")]
        Signed,
        #[literal("sizeof")]
        Sizeof,
        #[literal("static")]
        Static,
        #[literal("struct")]
        Struct,
        #[literal("switch")]
        Switch,
        #[literal("typedef")]
        Typedef,
        #[literal("union")]
        Union,
        #[literal("unsigned")]
        Unsigned,
        #[literal("void")]
        Void,
        #[literal("volatile")]
        Volatile,
        #[literal("while")]
        While,
        #[literal("_Alignas")]
        _Alignas,
        #[literal("_Alignof")]
        _Alignof,
        #[literal("_Atomic")]
        _Atomic,
        #[literal("_Bool")]
        _Bool,
        #[literal("_Complex")]
        _Complex,
        #[literal("_Generic")]
        _Generic,
        #[literal("_Imaginary")]
        _Imaginary,
        #[literal("_Noreturn")]
        _Noreturn,
        #[literal("_Static_assert")]
        _StaticAssert,
        #[literal("_Thread_local")]
        _ThreadLocal,
    }
);

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Keyword(KeywordKind),
    Identifier,
    Constant(ConstantKind),
    StringLiteral(String),
    Punctuator(PunctuatorKind),
    PPNumber,
    IntegerConstant(isize),
    FloatingConstant(f64),
    EnumerationConstant,
    CharacterConstant(char),
    Other,
}

#[derive(Debug, PartialEq)]
pub struct Token<P: AsRef<Path>> {
    kind: TokenKind,
    i: usize,
    width: usize,
    path: P,
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    NoMatch,
    InvalidNewLine,
    InvalidCharacter,
    EOF,
    NotSupported,
    MissingClosingQuote,
}

#[derive(Debug, PartialEq)]
enum Encoding {
    UTF8,
    Wide,
}

/// Return true if the character is an octal digit.
fn is_octal(c: char) -> bool {
    c == '0' || c == '1' || c == '2' || c == '3' || c == '4' || c == '5'
        || c == '6' || c == '7'
}

/// Parse an octal sequence from the chars.
fn parse_octal<'a>(c1: char, cp: &mut Peekable<Chars<'a>>) -> Option<(char, usize)> {
    // Calculate the octal number
    let result = if is_octal(c1) {
        if cp.peek().is_some() && is_octal(*cp.peek().unwrap()) {
            let c2 = cp.next().unwrap();
            if cp.peek().is_some() && is_octal(*cp.peek().unwrap()) {
                let c3 = cp.next().unwrap();
                let cval = c1.to_digit(8).unwrap() * 64 + c2.to_digit(8).unwrap() * 8
                           + c3.to_digit(8).unwrap();
                Some((cval, 3))
            } else {
                Some((c1.to_digit(8).unwrap() * 8 + c2.to_digit(8).unwrap(), 2))
            }
        } else {
            Some((c1.to_digit(8).unwrap(), 1))
        }
    } else {
        None
    };
    // Now try to get the character
    match result {
        Some((val, i)) => {
            match char::from_u32(val) {
                Some(c) => Some((c, i)),
                None => None,
            }
        }
        None => None,
    }
}

/// Parse a two-digit hex value.
fn parse_hex<'a>(cp: &mut Peekable<Chars<'a>>) -> Option<char> {
    if cp.peek().is_some() && cp.peek().unwrap().is_ascii_hexdigit() {
        let a = cp.next().unwrap();
        if cp.peek().is_some() && cp.peek().unwrap().is_ascii_hexdigit() {
            let b = cp.next().unwrap();
            let val = a.to_digit(16).unwrap() * 16 + b.to_digit(16).unwrap();
            char::from_u32(val)
        } else {
            None
        }
    } else {
        None
    }
}

/// Parse and return the unicode character of a \u{codepoint} escape.
fn parse_unicode4<'a>(cp: &mut Peekable<Chars<'a>>) -> Option<char> {
    let c1 = cp.next()?.to_digit(16)?;
    let c2 = cp.next()?.to_digit(16)?;
    let c3 = cp.next()?.to_digit(16)?;
    let c4 = cp.next()?.to_digit(16)?;
    char::from_u32(c1 * 4096 + c2 * 256 + c3 * 16 + c4)
}

/// Parse and return the unicode character of a \U{codepoint} escape.
fn parse_unicode8<'a>(cp: &mut Peekable<Chars<'a>>) -> Option<char> {
    let c1 = cp.next()?.to_digit(16)?;
    let c2 = cp.next()?.to_digit(16)?;
    let c3 = cp.next()?.to_digit(16)?;
    let c4 = cp.next()?.to_digit(16)?;
    let c5 = cp.next()?.to_digit(16)?;
    let c6 = cp.next()?.to_digit(16)?;
    let c7 = cp.next()?.to_digit(16)?;
    let c8 = cp.next()?.to_digit(16)?;
    char::from_u32(c1 * 268435456 + c2 * 16777216 + c3 * 1048576 + c4 * 65536
                   + c5 * 4096 + c6 * 256 + c7 * 16 + c8)
}

/// Parse a C char sequence.
pub fn parse_c_char_sequence<'a>(
    cp: &mut Peekable<Chars<'a>>,
    char_constant: bool,
) -> Option<(char, usize)> {
    if cp.peek().is_none() {
        return None;
    }
    match cp.next().unwrap() {
        '\\' => {
            if cp.peek().is_none() {
                return None;
            }
            let c = cp.next().unwrap();
            match c {
                // Simple escape sequence
                '\'' => Some(('\'', 2)),
                '"' => Some(('"', 2)),
                '?' => Some(('?', 2)),
                '\\' => Some(('\\', 2)),
                'a' => Some(('\x07', 2)),
                'b' => Some(('\x08', 2)),
                'f' => Some(('\x0C', 2)),
                'n' => Some(('\n', 2)),
                'r' => Some(('\r', 2)),
                't' => Some(('\t', 2)),
                'v' => Some(('\x0B', 2)),
                _ => {
                    if let Some((c, j)) = parse_octal(c, cp) {
                        // Octal digit sequence
                        Some((c, j + 1))
                    } else if c == 'x' {
                        if let Some(c) = parse_hex(cp) {
                            Some((c, 4))
                        } else {
                            None
                        }
                    // Parse unicode
                    } else if c == 'u' {
                        if let Some(c) = parse_unicode4(cp) {
                            Some((c, 6))
                        } else {
                            None
                        }
                    } else if c == 'U' {
                        if let Some(c) = parse_unicode8(cp) {
                            Some((c, 10))
                        } else {
                            None
                        }
                        // panic!("unicode escapes aren't supported yet");
                    } else {
                        None
                    }
                }
            }
        }
        // Check for an empty character constant
        '\'' if char_constant => {
            None
        }
        c => {
            Some((c, 1))
        }
    }
}

/// Parse a character constant.
pub fn parse_character_constant<'a>(
    mut cp: Peekable<Chars<'a>>,
) -> Result<(usize, Peekable<Chars<'a>>, TokenKind), LexerError> {
    if let Some('\'') = cp.next() {
        if let Some((c, i)) = parse_c_char_sequence(&mut cp, true) {
            if let Some('\'') = cp.next() {
                Ok((i + 2, cp, TokenKind::CharacterConstant(c)))
            } else {
                Err(LexerError::MissingClosingQuote)
            }
        } else {
            Err(LexerError::InvalidCharacter)
        }
    } else {
        Err(LexerError::NoMatch)
    }
}

/// Parse a string.
pub fn parse_string<'a>(
    mut cp: Peekable<Chars<'a>>,
) -> Result<(usize, Peekable<Chars<'a>>, TokenKind), LexerError> {
    let mut i = 0;
    // Find the encoding
    let encoding = if let Some(&'u') = cp.peek() {
        cp.next();
        i += 1;
        if let Some(&'8') = cp.peek() {
            cp.next();
            i += 1;
            Encoding::Wide
        } else {
            Encoding::UTF8
        }
    } else if let Some(&'U') = cp.peek() {
        cp.next();
        i += 1;
        Encoding::Wide
    } else if let Some(&'L') = cp.peek() {
        cp.next();
        i += 1;
        Encoding::Wide
    } else {
        Encoding::UTF8
    };
    // Start character
    let start = match cp.peek() {
        Some(c) if *c == '<' || *c == '"' => {
            Some(c)
        }
        _ => {
            None
        },
    };
    if start.is_none() {
        return Err(LexerError::NoMatch);
    }
    if encoding == Encoding::Wide {
        return Err(LexerError::NotSupported);
    }
    // Determine the end mark
    let end = if Some(&'<') == start { '>' } else { '"' };
    cp.next();
    i += 1;
    // Continue until the end
    let mut s = String::new();
    while cp.peek().is_some() {
        let c = cp.peek().unwrap();
        match *c {
            c if c == end => {
                cp.next();
                i += 1;
                return Ok((i, cp, TokenKind::StringLiteral(s)));
            }
            '\n' => return Err(LexerError::InvalidNewLine),
            _ => {
                if let Some((c, j)) = parse_c_char_sequence(&mut cp, false) {
                    i += j;
                    s.push(c);
                } else {
                    return Err(LexerError::InvalidCharacter);
                }
            }
        }
        // let c = parse_c_char_sequence(cp.clone());
/*
        match *c {
            '\\' => {
                cp.next();
                i += 1;
                let c = cp.peek();
                if c.is_none() {
                    return Err(LexerError::EOF);
                }
                let c = *c.unwrap();
                match c {
                    '"' => {
                        cp.next();
                        i += 1;
                        s.push('"');
                    }
                    _ => {
                        cp.next();
                        i += 1;
                        s.push('\\');
                        s.push(c);
                    }
                }
            }
            c if c == end => {
                cp.next();
                i += 1;
                return Ok((i, cp, TokenKind::StringLiteral(s)));
            }
            '\n' => {
                return Err(LexerError::InvalidNewLine);
            }
            _ => {
                s.push(*c);
                cp.next();
                i += 1;
            }
        }
*/
    }
    return Err(LexerError::EOF);
}

/// Parse an identifier.
pub fn parse_identifier<'a>(
    mut cp: Peekable<Chars<'a>>,
) -> Result<(usize, Peekable<Chars<'a>>, TokenKind), LexerError> {
    let mut i = 0;
    if cp.peek().is_none() || !cp.peek().unwrap().is_ascii_alphabetic() {
        return Err(LexerError::NoMatch);
    }
    cp.next();
    i += 1;
    while cp.peek().is_some() && (cp.peek().unwrap().is_ascii_alphabetic()
                                  || cp.peek().unwrap().is_ascii_digit()
                                  || cp.peek().unwrap() == &'_')
    {
        cp.next();
        i += 1;
    }
    Ok((i, cp, TokenKind::Identifier))
}

/// Parse an integer constant.
pub fn parse_integer_constant<'a>(
    mut cp: Peekable<Chars<'a>>,
) -> Result<(usize, Peekable<Chars<'a>>, TokenKind), LexerError> {
/*
    if let Some(c) = cp.next() {
        if c == '0' {
        } else if c.is_ascii_digit() {
        } else {
            None
        }
    } else {
*/
        Err(LexerError::NoMatch)
/*
    }
*/
}

/// Get the next token from the stream.
/*
fn next_token<'a>(mut cp: Peekable<Chars<'a>>) -> Result<Token, LexerError> {
    let mut best = None;
    while cp.peek() == Some(&' ') || cp.peek() == Some(&'\t') {
        cp.next();
    }
    // parse_(cp.clone())
    Ok()
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_string_simple() {
        let s = "\"123 abc\"";

        let (i, mut p, k) = parse_string(s.chars().peekable()).unwrap();
        assert_eq!(i, 9);
        assert_eq!(p.peek(), None);
        assert_eq!(k, TokenKind::StringLiteral("123 abc".to_string()));
    }

    #[test]
    fn parse_string_complex() {
        let s = "u\"123 \\\" a bc\"";

        let (i, mut p, k) = parse_string(s.chars().peekable()).unwrap();
        assert_eq!(i, 14);
        assert_eq!(p.peek(), None);
        assert_eq!(k, TokenKind::StringLiteral("123 \" a bc".to_string()));
    }

    #[test]
    fn parse_string_simple_escape() {
        let s = "\"\\n\\\\\\r\\t\\a\\b\\?\"";

        let (i, mut p, k) = parse_string(s.chars().peekable()).unwrap();
        assert_eq!(i, 16);
        assert_eq!(p.peek(), None);
        assert_eq!(k, TokenKind::StringLiteral("\n\\\r\t\x07\x08?".to_string()));
    }

    #[test]
    fn parse_string_octal() {
        let s = "\"\\0\\47\\175\"";

        let (i, mut p, k) = parse_string(s.chars().peekable()).unwrap();
        assert_eq!(i, 11);
        assert_eq!(p.peek(), None);
        assert_eq!(k, TokenKind::StringLiteral("\0'}".to_string()));
    }

    #[test]
    fn parse_string_hex() {
        let s = "\"\\x20\\x79\\x3A\\x3a\"";

        let (i, mut p, k) = parse_string(s.chars().peekable()).unwrap();
        assert_eq!(i, 18);
        assert_eq!(p.peek(), None);
        assert_eq!(k, TokenKind::StringLiteral(" y::".to_string()));
    }

    #[test]
    fn parse_string_unicode() {
        let s = "\"\\u0628\\U00016F37\"";

        let (i, mut p, k) = parse_string(s.chars().peekable()).unwrap();
        assert_eq!(k, TokenKind::StringLiteral("ب𖼷".to_string()));
        assert_eq!(i, 18);
        assert_eq!(p.peek(), None);
    }

    #[test]
    fn parse_character_constant_simple() {
        let s = "'U'";

        let (i, mut p, k) = parse_character_constant(s.chars().peekable()).unwrap();
        assert_eq!(k, TokenKind::CharacterConstant('U'));
        assert_eq!(i, 3);
        assert_eq!(p.peek(), None);
    }

    #[test]
    fn parse_character_constant_unicode() {
        let s = "'\\u0628'";

        let (i, mut p, k) = parse_character_constant(s.chars().peekable()).unwrap();
        assert_eq!(k, TokenKind::CharacterConstant('ب'));
        assert_eq!(i, 8);
        assert_eq!(p.peek(), None);
    }

    #[test]
    fn parse_integer_constant_zero() {
        let s = "0";

        let (i, mut p, k) = parse_integer_constant(s.chars().peekable()).unwrap();
        assert_eq!(k, TokenKind::IntegerConstant(0));
        assert_eq!(i, 1);
        assert_eq!(p.peek(), None);
    }

    #[test]
    fn parse_integer_constant_nonzero() {
        let s = "123";

        let (i, mut p, k) = parse_integer_constant(s.chars().peekable()).unwrap();
        assert_eq!(k, TokenKind::IntegerConstant(123));
        assert_eq!(i, 3);
        assert_eq!(p.peek(), None);
    }

    #[test]
    fn parse_integer_constant_octal() {
        let s = "077";

        let (i, mut p, k) = parse_integer_constant(s.chars().peekable()).unwrap();
        assert_eq!(k, TokenKind::IntegerConstant(63));
        assert_eq!(i, 3);
        assert_eq!(p.peek(), None);
    }

    #[test]
    fn parse_integer_constant_hex() {
        let s = "0xFF";

        let (i, mut p, k) = parse_integer_constant(s.chars().peekable()).unwrap();
        assert_eq!(k, TokenKind::IntegerConstant(255));
        assert_eq!(i, 4);
        assert_eq!(p.peek(), None);
    }
}
