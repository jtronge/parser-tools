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
}

type Result<T> = std::result::Result<T, Error>;

pub struct PreprocessorOptions {
    pub defs: HashMap<String, Macro>,
    pub include_paths: Vec<String>,
}

/// C Preprocessor.
pub struct Preprocessor {
    state: Rc<RefCell<State>>,
    buffer: VecDeque<Token>,
    ready: VecDeque<Token>,
    directive_pass: DirectivePass,
}

impl Preprocessor {
    /// Initialize a new C Preprocessor tool.
    pub fn new(path: &str, opts: PreprocessorOptions) -> Preprocessor {
        // let state = State::new(path, opts);
        let state = Rc::new(RefCell::new(State::new(path)));
        let buffer = VecDeque::new();
        let ready = VecDeque::new();
        let directive_pass = DirectivePass::new(Rc::clone(&state));
        Preprocessor {
            state,
            buffer,
            ready,
            directive_pass,
        }
    }
}

impl Iterator for Preprocessor {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ready.len() == 0 {
            if let Err(e) = scan::scan(
                Rc::clone(&self.state),
                &mut self.directive_pass,
                &mut self.ready,
            ) {
                return Some(Err(e));
            }
        }
        self.ready
            .pop_front()
            .map(|rtok| Ok(rtok))
    }
}
