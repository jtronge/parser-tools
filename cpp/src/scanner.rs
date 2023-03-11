use std::borrow::Borrow;
use std::collections::VecDeque;
use std::rc::Rc;
use ctokens::Token;
use crate::{Result, Error};
use crate::cmacro::Macro;
use crate::state::State;


pub fn scan(
    it: impl Iterator<Item=Token>,
    out: &mut Vec<Token>,
) -> Result<usize> {
    Ok(0)
}

pub struct Scanner {
    stack: VecDeque<ScannerItem>,
    pub state: State,
    pub ready: VecDeque<Token>,
}

impl Scanner {
    pub fn new(state: State) -> Scanner {
        Scanner {
            stack: VecDeque::new(),
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

/*
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

*/
    /// Scan for more tokens.
    pub fn scan(&mut self) -> Result<()> {
        match self.state.next() {
            Some(tok) => {
                let tok = tok?;
                if let Some(mac) = self.find_macro(&tok) {
                    // TODO
                    panic!("Process macro");
                } else {
                    self.ready.push_back(tok);
                }
            }
            _ => (),
        }
        Ok(())
    }
}

/// Get the args for a functional macro.
fn get_args<'a>(ti: impl Iterator<Item=Token> + 'a) -> Result<Vec<Vec<Token>>> {
    let mut args = vec![];
    let mut ti = ti.peekable();
    // Assume opening parenthesis
    let _ = ti.next();
    loop {
        println!("QUI: {:?}!\n", ti.peek());
        let mut arg = vec![];
        let mut paren_bal = 0;
        loop {
            match ti.peek() {
                Some(Token::LParen) => paren_bal += 1,
                Some(Token::RParen) => {
                    if paren_bal == 0 {
                        break;
                    } else {
                        paren_bal -= 1;
                    }
                }
                Some(Token::Comma) => {
                    if paren_bal == 0 {
                        break;
                    }
                }
                None => break,
                _ => (),
            }
            arg.push(ti.next().unwrap().clone());
        }
        match ti.peek() {
            None => return Err(Error::MissingClosingParenMacroCall),
            Some(Token::Comma) => {
                let _ = ti.next();
                args.push(arg);
            }
            Some(Token::RParen) => {
                if arg.len() > 0 || args.len() > 0 {
                    args.push(arg);
                }
                let _ = ti.next();
                break;
            }
            _ => (),
        }
    }
    Ok(args)
}

enum ScannerItem {
    /// Token that could be further expanded/replaced
    Token(Token),
    /// Arguments of function macro to expand
    FnMacro {
        name: String,
        args: Box<Vec<ScannerItem>>
    },
    Argument(Vec<Token>),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_args() {
        let toks = vec![
            Token::LParen,
            Token::RParen,
        ];

        let args = get_args(toks.iter().map(|tok| tok.clone())).unwrap();

        assert_eq!(args.len(), 0);
    }

    #[test]
    fn one_arg() {
        let toks = vec![
            Token::LParen,
            Token::Ident("Ciao, mondo".to_string()),
            Token::RParen,
        ];

        let args = get_args(toks.iter().map(|tok| tok.clone())).unwrap();

        assert_eq!(args.len(), 1);
        assert_eq!(args[0], vec![Token::Ident("Ciao, mondo".to_string())]);
    }

    #[test]
    fn some_more_args() {
        let toks = vec![
            Token::LParen,
            Token::Ident("abc".to_string()),
            Token::Comma,
            Token::LParen,
            Token::StringLit("something".to_string()),
            Token::RParen,
            Token::RParen,
        ];

        let args = get_args(toks.iter().map(|tok| tok.clone())).unwrap();

        assert_eq!(args.len(), 2);
        assert_eq!(args[0], vec![
            Token::Ident("abc".to_string()),
        ]);
        assert_eq!(args[1], vec![
            Token::LParen,
            Token::StringLit("something".to_string()),
            Token::RParen,
        ]);
    }
}
