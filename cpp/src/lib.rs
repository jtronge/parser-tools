use std::collections::{
    HashMap,
    VecDeque,
};
use std::path::Path;

mod atom;
mod cmacro;
mod state;
mod line_processor;

pub use atom::Atom;
use cmacro::Macro;
use state::State;

#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    IOError(String),
}

type Result<T> = std::result::Result<T, Error>;

pub struct PreprocessorOptions {
    pub defs: HashMap<String, Macro>,
    pub include_paths: Vec<String>,
}

/// C Preprocessor.
pub struct Preprocessor {
    state: State,
    ready: VecDeque<Atom>,
}

impl Preprocessor {
    /// Initialize a new C Preprocessor tool.
    pub fn new(path: &str, opts: PreprocessorOptions) -> Preprocessor {
        Preprocessor {
            state: State::new(path, opts),
            ready: VecDeque::new(),
        }
    }

    /// Get more atoms from the state and replace tokens.
    fn get_more(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Iterator for Preprocessor {
    type Item = Result<Atom>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ready.len() == 0 {
            // Need to refill the ready buffer
            if let Err(e) = self.get_more() {
                return Some(Err(e))
            }
        }
        if self.ready.len() == 0 {
            // No more tokens
            None
        } else {
            self.ready
                .pop_front()
                .map(|a| Ok(a))
        }
    }
}
