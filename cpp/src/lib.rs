use std::collections::{
    HashMap,
    VecDeque,
};
use std::path::Path;
use ctokens::{
    Token,
    TokenError,
};

mod cmacro;
mod state;
mod line_processor;
mod scanner;

use cmacro::Macro;
use state::State;
use scanner::Scanner;

#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    IOError(String),
    ParserError(String),
    TokenError(TokenError),
    /// Missing a closing parenthesis for a macro all
    MissingClosingParenMacroCall,
    InvalidMacro,
}

type Result<T> = std::result::Result<T, Error>;

pub struct PreprocessorOptions {
    pub defs: HashMap<String, Macro>,
    pub include_paths: Vec<String>,
}

/// C Preprocessor.
pub struct Preprocessor {
    scanner: Scanner,
}

impl Preprocessor {
    /// Initialize a new C Preprocessor tool.
    pub fn new(path: &str, opts: PreprocessorOptions) -> Preprocessor {
        let state = State::new(path, opts);
        Preprocessor {
            scanner: Scanner::new(state),
        }
    }
}

impl Iterator for Preprocessor {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.scanner.ready.len() == 0 {
            // Need to refill the ready buffer
            if let Err(e) = self.scanner.scan() {
                return Some(Err(e));
            }
        }
        if self.scanner.ready.len() == 0 {
            // No more tokens
            None
        } else {
            self.scanner.ready
                .pop_front()
                .map(|a| Ok(a))
        }
    }
}
