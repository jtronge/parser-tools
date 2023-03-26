use std::collections::{
    HashMap,
    VecDeque,
};
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;
use ctokens::{
    Token,
    TokenError,
};

mod cmacro;
mod state;
mod line_processor;
mod directive;
mod scan;

use cmacro::Macro;
use state::State;
use directive::DirectivePass;

#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    IOError(String),
    ParserError(String),
    TokenError(TokenError),
    /// Missing a closing parenthesis for a macro all
    MissingClosingParenMacroCall,
    InvalidMacro,
    /// Invalid condition during the scan step
    ScanInternalError,
    /// Invalid number of arguments to a macro
    InvalidMacroArgumentCount,
}

type Result<T> = std::result::Result<T, Error>;

pub struct PreprocessorOptions {
    pub defs: HashMap<String, Macro>,
    pub include_paths: Vec<String>,
}

/// C Preprocessor.
pub struct Preprocessor {
    toks: Vec<Token>,
    i: usize,
}

impl Preprocessor {
    /// Initialize a new C Preprocessor tool.
    pub fn new(path: &str, opts: PreprocessorOptions) -> Preprocessor {
        // let state = State::new(path, opts);
        let state = Rc::new(RefCell::new(State::new(path)));
        let mut directive_pass = DirectivePass::new(Rc::clone(&state));
        let mut toks = vec![];
        scan::scan(state, &mut directive_pass, &mut toks);
        Preprocessor {
            toks,
            i: 0,
        }
    }
}

impl Iterator for Preprocessor {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > self.toks.len() {
            None
        } else {
            Some(Ok(self.toks[self.i].clone()))
        }
    }
}
