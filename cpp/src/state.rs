use std::collections::VecDeque;
use std::fs::File;
use crate::{
    Error,
    Result,
    PreprocessorOptions,
};
use crate::atom::Atom;
use crate::line_processor::LineProcessor;

/// State at an instant in time of the C preprocessor.
pub struct State {
    opts: PreprocessorOptions,
    /// Line processor
    lp: LineProcessor<File>,
    buffer: VecDeque<Atom>,
    lno: usize,
}

impl State {
    pub fn new(path: &str, opts: PreprocessorOptions) -> State {
        let fp = File::open(path).expect("Could not open file");
        State {
            opts,
            lp: LineProcessor::new(fp),
            buffer: VecDeque::new(),
            lno: 0,
        }
    }

    /// Read in a line, processing all comments and line continuations.
    fn read_line(&mut self, line: &mut String) -> bool {
        false
    }

    /// Tokenize the line, reading tokens into self.buffer
    fn tokenize(&mut self, line: &str) {
    }

    /// Handle a directive.
    fn directive(&mut self, line: &str) {
    }
}

impl Iterator for State {
    type Item = Result<Atom>;

    fn next(&mut self) -> Option<Self::Item> {
        // First check if the buffer has something
        match self.buffer.pop_front() {
            Some(atom) => return Some(Ok(atom)),
            None => (),
        }

        // Continue while there's more to process or until we put something in
        // the buffer.
        let mut line = String::new();
        loop {
            if self.read_line(&mut line) {
                if is_directive(&line) {
                    self.directive(&line);
                } else {
                    self.tokenize(&line);
                }
                // self.directive() also could have pushed a special "Prgama"
                // token
                match self.buffer.pop_front() {
                    // Got something, so return it
                    Some(atom) => return Some(Ok(atom)),
                    // Didn't get anything, but more lines to process
                    None => (),
                }
            } else {
                break;
            }
        }
        None
    }
}

/// Check if the line is a directive.
fn is_directive(line: &str) -> bool {
    // TODO
    false
}
