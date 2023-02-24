use std::collections::VecDeque;
use std::fs::File;
use ctokens::{Token, TokenIter};
use nom::{
    IResult,
    character::complete::{
        char,
        space0,
        space1,
        alpha1,
        alphanumeric0,
        alphanumeric1,
    },
    multi::{
        many0,
        separated_list0,
    },
    bytes::complete::tag,
    error::VerboseError,
    sequence::pair,
    branch::alt,
};
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
        /// Match and return an identifier
        fn ident(i: &str) -> IResult<&str, &str> {
            let start = i;
            let (i, _) = alt((alpha1, tag("_")))(i)?;
            alt((tag("_"), alphanumeric1))(i)
                .map(|(i, rest)| (i, &start[..rest.len() + 1]))
        }

        fn match_initial_define(line: &str) -> IResult<&str, &str> {
            let (i, _) = space0(line)?;
            let (i, _) = char('#')(i)?;
            let (i, _) = space0(i)?;
            let (i, _) = tag("define")(i)?;
            space1(i)
        }

        fn match_define_obj(line: &str) -> IResult<&str, &str> {
            let (i, _) = match_initial_define(line)?;
            let (i, name) = ident(i)?;
            space1(i).map(|(i, _)| (i, name))
        }

        /// Match a function-like macro.
        fn match_define_fn(line: &str) -> IResult<&str, (&str, Vec<&str>)> {
            let (i, _) = match_initial_define(line)?;
            let (i, name) = ident(i)?;
            // No space between the name and the parenthesis
            let (i, _) = char('(')(i)?;
            // Get the arguments
            // TODO: This is incorrect
            let (i, args) = separated_list0(
                pair(space0, pair(char(','), space0)),
                ident,
            )(i)?;
            char(')')(i).map(|(i, _)| (i, (name, args)))
        }

        if let Ok((i, name)) = match_define_obj(line) {
            eprintln!("Got define for object macro: {}", name);
            Ok(())
        } else if let Ok((i, (name, args))) = match_define_fn(line) {
            eprintln!("Got define for function-like macro: {}({:?})", name, args);
            Ok(())
        } else {
            Err(Error::InvalidMacro)
        }
    }

    /// Tokenize a non-directive line of the source and push the tokens onto
    /// the end of the buffer.
    fn tokenize(&mut self, line: &str) -> Result<()> {
        for res in TokenIter::new(line) {
            let tok = res.map_err(|err| Error::TokenError(err))?;
            self.buffer.push_back(tok);
        }
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
