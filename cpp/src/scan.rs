// use std::borrow::Borrow;
use std::collections::{
    HashMap,
    VecDeque,
};
use std::rc::Rc;
use std::cell::RefCell;
use ctokens::Token;
use crate::{
    Result,
    Error,
};
use crate::cmacro::Macro;
use crate::state::State;

pub fn scan<'a>(
    state: Rc<RefCell<State>>,
    mut ti: impl Iterator<Item=Result<Token>> + 'a,
    buffer: &mut VecDeque<Token>,
    ready: &mut VecDeque<Token>,
) -> Result<()> {
    if let Some(tok) = ti.next() {
        let tok = tok?;
        if let Some(mac) = state.borrow().find_macro(&tok) {
            // TODO: Implement macro replacement
        } else {
            // Otherwise push it onto the ready buffer
            ready.push_back(tok);
        }
    }
    Ok(())
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
