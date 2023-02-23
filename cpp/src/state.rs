use std::collections::VecDeque;
use std::fs::File;
use ctokens::Token;
use nom::{
    IResult,
    character::complete::{
        char,
        space0,
        space1,
        alphanumeric0,
        alphanumeric1,
    },
    multi::many0,
    bytes::complete::tag,
    error::VerboseError,
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
        fn match_initial_define(line: &str) -> IResult<&str, &str> {
            let (i, _) = space0(line)?;
            let (i, _) = char('#')(i)?;
            let (i, _) = space0(i)?;
            let (i, _) = tag("define")(i)?;
            space1(i)
        }

        fn match_define_obj(line: &str) -> IResult<&str, &str> {
            let (i, _) = match_initial_define(line)?;
            let (i, name) = alphanumeric1(i)?;
            space1(i).map(|(i, _)| (i, name))
        }

        /// Match a function-like macro.
        fn match_define_fn(line: &str) -> IResult<&str, (&str, Vec<&str>)> {
            let (i, _) = match_initial_define(line)?;
            let (i, name) = alphanumeric1(i)?;
            let (i, _) = char('(')(i)?;
            // Get the arguments
            // let mut args = vec![];
/*
            let (i, _) = space0(i)?;
            loop {
                if let Ok((tmp_i, argname)) = alphanumeric1::<&str, &str>(i) {
                    args.push(argname);
                    let (tmp_i, _) = space0(tmp_i)?;
                    i = tmp_i;
                    if let Ok((tmp_i, _)) = char::<&str, &str>(',')(tmp_i) {
                        let (tmp_i, _) = space0(tmp_i)?;
                        i = tmp_i;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
*/
/*
            let (i, args) = many0(|i| {
                let (i, _) = space0(i)?;
                let (i, argname) = alphanumeric1(i)?;
                let (i, _) = space0(i)?;
                char(',')
            })?;
*/
            // TODO: This is incorrect
            /* let (i, args) = many0(alphanumeric1)?; */
            char(')')(i).map(|(i, _)| (i, (name, vec![])))
        }

        if let Ok((i, name)) = match_define_obj(line) {
            panic!("Got define for object macro: {}", name);
        }
        if let Ok((i, (name, args))) = match_define_fn(line) {
            panic!("Got define for function-like macro: {}({:?})", name, args);
        }
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
