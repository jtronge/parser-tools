use std::collections::HashMap;
use std::path::Path;

mod atom;
mod cmacro;
mod state;

pub use atom::Atom;
use cmacro::Macro;
use state::State;

pub struct PreprocessorOptions {
    pub defs: HashMap<String, Macro>,
    pub include_paths: Vec<String>,
}

/// C Preprocessor.
pub struct Preprocessor {
    state: State,
}

impl Preprocessor {
    /// Initialize a new C Preprocessor tool.
    pub fn new(path: &str, opts: PreprocessorOptions) -> Preprocessor {
        Preprocessor {
            state: State::new(path, opts),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum PreprocessorError {}

type Result<T> = std::result::Result<T, PreprocessorError>;

impl Iterator for Preprocessor {
    type Item = Result<Atom>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
