use std::collections::VecDeque;
use std::fs::File;
use ctokens::Token;
use crate::{
    Error,
    Result,
    PreprocessorOptions,
};
use crate::line_processor::LineProcessor;

/// State at an instant in time of the C preprocessor.
pub struct State {
    opts: PreprocessorOptions,
    /// Line processor
    lp: LineProcessor<File>,
    buffer: VecDeque<Token>,
}

impl State {
    pub fn new(path: &str, opts: PreprocessorOptions) -> State {
        let fp = File::open(path).expect("Could not open file");
        State {
            opts,
            lp: LineProcessor::new(fp),
            buffer: VecDeque::new(),
        }
    }

    /// Process a directive line and update the state.
    fn process_directive(&mut self, line: &str) -> Result<()> {
        Ok(())
    }

    /// Tokenize a non-directive line of the source and push the tokens onto
    /// the end of the buffer.
    fn tokenize(&mut self, line: &str) -> Result<()> {
        Ok(())
    }
}

impl Iterator for State {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        // First check if the buffer has something
        if let Some(token) = self.buffer.pop_front() {
            return Some(Ok(token));
        }
        // Continue while there's more to process or until we put something in
        // the buffer.
        loop {
            let line = match self.lp.next() {
                Some(Ok(l)) => l,
                // TODO: Need to propagate this error up
                Some(Err(_)) => return None,
                None => return None,
            };

            if is_directive(&line) {
                if let Err(err) = self.process_directive(&line) {
                    return Some(Err(err));
                }
            } else {
                if let Err(err) = self.tokenize(&line) {
                    return Some(Err(err));
                }
            }

            // The directive could have also produced tokens from a pragma
            if self.buffer.len() > 0 {
                break;
            }
        }

        self.buffer
            .pop_front()
            .map(|token| Ok(token))
    }
}

use nom::{
    IResult,
    character::complete::{
        space0,
        char,
        alphanumeric0,
    },
    error::VerboseError,
};

/// Check if the line is a directive.
fn is_directive(line: &str) -> bool {
    fn inner_is_directive(line: &str) -> IResult<&str, &str> {
        let (i, _) = space0(line)?;
        let (i, _) = char('#')(i)?;
        let (i, _) = space0(i)?;
        alphanumeric0(i)
    };

    match inner_is_directive(line) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn not_directive() {
        assert!(!is_directive("abc 123"));
    }

    #[test]
    fn directive_include() {
        assert!(is_directive("#include \"some-file.h\""));
    }

    #[test]
    fn directive_pragma() {
        assert!(is_directive("#pragma ..."));
    }
}
