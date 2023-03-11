use std::collections::{
    HashMap,
    VecDeque,
};
use std::path::Path;
use std::rc::Rc;
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
use directive::process_directives;

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
    state: State,
    buffer: VecDeque<Token>,
    ready: VecDeque<Token>,
}

impl Preprocessor {
    /// Initialize a new C Preprocessor tool.
    pub fn new(path: &str, opts: PreprocessorOptions) -> Preprocessor {
        // let state = State::new(path, opts);
        let state = State::new(path);
        let buffer = VecDeque::new();
        let ready = VecDeque::new();
        Preprocessor {
            state,
            buffer,
            ready,
        }
    }

    fn find_macro(&self, tok: &Token) -> Option<Rc<Macro>> {
        if let Token::Ident(ref name) = tok {
            if let Some(mac) = self.state.defines.get(name) {
                Some(Rc::clone(mac))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn scan_next(&mut self) -> Result<()> {
        process_directives(&mut self.state, &mut self.buffer)?;
        if let Some(tok) = self.buffer.pop_front() {
            if let Some(mac) = self.find_macro(&tok) {
                panic!("found macro");
            } else {
                self.ready.push_back(tok);
            }
        }
        Ok(())
    }
}

impl Iterator for Preprocessor {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ready.len() == 0 {
            if let Err(e) = self.scan_next() {
                return Some(Err(e));
            }
        }
        self.ready
            .pop_front()
            .map(|rtok| Ok(rtok))
    }
}
