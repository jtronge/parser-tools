use std::collections::VecDeque;
use std::fs::File;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use ctokens::{Token, TokenIter};
use matching::cident;
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
use crate::cmacro::Macro;
use crate::state::State;

pub struct DirectivePass {
    /// Line processor
    lp: LineProcessor<File>,
    buffer: VecDeque<Token>,
    state: Rc<RefCell<State>>,
}

impl DirectivePass {
    pub fn new(state: Rc<RefCell<State>>) -> DirectivePass {
        let fp = File::open(&state.borrow().files.iter().nth(0).unwrap())
            .expect("Could not open file");
        DirectivePass {
            lp: LineProcessor::new(fp),
            buffer: VecDeque::new(),
            state,
        }
    }

    /// Process a directive line and update the state.
    fn process_directive(&mut self, line: &str) -> Result<()> {
        let state = self.state.borrow_mut();
        fn match_initial_define(line: &str) -> IResult<&str, &str> {
            let (i, _) = space0(line)?;
            let (i, _) = char('#')(i)?;
            let (i, _) = space0(i)?;
            let (i, _) = tag("define")(i)?;
            space1(i)
        }

        fn match_define_obj(line: &str) -> IResult<&str, &str> {
            let (i, _) = match_initial_define(line)?;
            let (i, name) = cident(i)?;
            space1(i).map(|(i, _)| (i, name))
        }

        /// Match a function-like macro.
        fn match_define_fn(line: &str) -> IResult<&str, (&str, Vec<&str>)> {
            let (i, _) = match_initial_define(line)?;
            let (i, name) = cident(i)?;
            // No space between the name and the parenthesis
            let (i, _) = char('(')(i)?;
            // Get the arguments
            // TODO: This is incorrect
            let (i, args) = separated_list0(
                pair(space0, pair(char(','), space0)),
                cident,
            )(i)?;
            char(')')(i).map(|(i, _)| (i, (name, args)))
        }

        if let Ok((i, name)) = match_define_obj(line) {
            let toks = tokenize(i)?;
            self.state
                .borrow_mut()
                .defines
                .insert(name.to_string(), Rc::new(Macro::Object(toks)));
            eprintln!("Got define for object macro: {}", name);
            Ok(())
        } else if let Ok((i, (name, args))) = match_define_fn(line) {
            eprintln!("Got define for function-like macro: {}({:?})", name, args);
            let toks = tokenize(i)?;
            let args: Vec<String> = args
                .iter()
                .map(|arg| arg.to_string())
                .collect();
            self.state
                .borrow_mut()
                .defines
                .insert(
                    name.to_string(),
                    Rc::new(Macro::Function(args, toks)),
                );
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

impl Iterator for DirectivePass {
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

fn tokenize(i: &str) -> Result<Vec<Token>> {
    let mut toks = vec![];
    for res in TokenIter::new(i) {
        let tok = res.map_err(|err| Error::TokenError(err))?;
        toks.push(tok);
    }
    Ok(toks)
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
