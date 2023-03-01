use std::borrow::Borrow;
use std::collections::VecDeque;
use std::rc::Rc;
use ctokens::Token;
use crate::{Result, Error};
use crate::cmacro::Macro;
use crate::state::State;

pub struct Scanner {
    pub state: State,
    pub ready: VecDeque<Token>,
}

impl Scanner {
    pub fn new(state: State) -> Scanner {
        Scanner {
            state,
            ready: VecDeque::new(),
        }
    }

    /// Check if this token matches a macro name.
    fn find_macro(&self, tok: &Token) -> Option<Rc<Macro>> {
        match tok {
            Token::Ident(ref name) => self.state.find_macro(name),
            _ => None,
        }
    }

    /// Handle the macro replacement and scanning operation.
    fn handle_macro(&mut self, mac: Rc<Macro>) -> Result<()> {
        match &*mac.borrow() {
            Macro::Object(toks) => {
                // NOTE: This isn't doing secondary scanning/replacement
                self.ready.extend(toks.iter().map(|tok| tok.clone()));
            }
            Macro::Function(args, toks) => (),
        }
        Ok(())
    }

    /// Scan for more tokens.
    pub fn scan(&mut self) -> Result<()> {
        if let Some(tok) = self.state.next() {
            let tok = tok?;
            if let Some(mac) = self.find_macro(&tok) {
                self.handle_macro(mac)?;
            } else {
                self.ready.push_back(tok);
            }
        }
        Ok(())
    }
}

fn get_args(ti: impl Iterator<Item=Token>) -> Result<Vec<Vec<Token>>> {
    let mut args = vec![];
    let mut ti = ti.peekable();
    loop {
        let mut arg = vec![];
        loop {
            match ti.peek() {
                Some(Token::Comma) | Some(Token::RParen) | None => break,
                _ => (),
            }
            arg.push(ti.next().unwrap());
        }
        match ti.peek() {
            None => return Err(Error::MissingClosingParenMacroCall),
            Some(Token::RParen) => break,
            _ => (),
        }
    }
    Ok(args)
}
