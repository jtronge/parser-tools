use std::collections::HashMap;
use ctokens::Token;
use cpp::{
    Preprocessor,
    PreprocessorOptions,
};
mod common;
use common::resource;

#[test]
fn empty() {
    let src = resource("empty.c");
    let opts = PreprocessorOptions {
        defs: HashMap::new(),
        include_paths: vec![],
    };
    let mut pp = Preprocessor::new(&src, opts);
    assert_eq!(pp.next(), None);
}

#[test]
fn xyz() {
    let src = resource("xyz.c");
    let opts = PreprocessorOptions {
        defs: HashMap::new(),
        include_paths: vec![],
    };
    let mut pp = Preprocessor::new(&src, opts);
    assert_eq!(pp.next(), Some(Ok(Token::Ident("xyz".to_string()))));
    assert_eq!(pp.next(), Some(Ok(Token::Ident("xyz".to_string()))));
    assert_eq!(pp.next(), None);
}

#[test]
fn concat_str() {
    let src = resource("concat-str.c");
    let opts = PreprocessorOptions {
        defs: HashMap::new(),
        include_paths: vec![],
    };
    let mut pp = Preprocessor::new(&src, opts);
    assert_eq!(pp.next(), Some(Ok(Token::Ident("Ajoin".to_string()))));
    assert_eq!(pp.next(), Some(Ok(Token::LParen)));
    assert_eq!(pp.next(), Some(Ok(Token::Ident("B".to_string()))));
    assert_eq!(pp.next(), Some(Ok(Token::Comma)));
    assert_eq!(pp.next(), Some(Ok(Token::Ident("C".to_string()))));
    assert_eq!(pp.next(), Some(Ok(Token::RParen)));
    assert_eq!(pp.next(), Some(Ok(Token::StringLit("join(A, B)".to_string()))));
    assert_eq!(pp.next(), None);
}
