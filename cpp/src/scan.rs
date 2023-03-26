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

// TODO: Maybe there should be a # and ## operator here as well?
enum ScanItem {
    FnMacro(Rc<Macro>),
    Arg,
    Token(Token),
}

pub fn scan<'a>(
    state: Rc<RefCell<State>>,
    mut ti: impl Iterator<Item=Result<Token>> + 'a,
    ready: &mut Vec<Token>,
) -> Result<()> {
    Ok(())
}

/// Fill buf with the args for a functional macro, adding barrier items as
/// needed (ScanItem::Arg). Return the number of arguments processed or error.
fn get_args<'a>(
    ti: impl Iterator<Item=Result<ScanItem>> + 'a,
    buf: &mut VecDeque<ScanItem>,
) -> Result<usize> {
    let mut argc = 0;
    let mut ti = ti.peekable();
    // Assume opening parenthesis
    let _ = ti.next();
    loop {
        let mut arg = vec![];
        let mut paren_bal = 0;
        loop {
            match ti.peek() {
                Some(Ok(ScanItem::Token(Token::LParen))) => paren_bal += 1,
                Some(Ok(ScanItem::Token(Token::RParen))) => {
                    if paren_bal == 0 {
                        break;
                    } else {
                        paren_bal -= 1;
                    }
                }
                Some(Ok(ScanItem::Token(Token::Comma))) => {
                    if paren_bal == 0 {
                        break;
                    }
                }
                Some(Ok(ScanItem::Token(_))) => (),
                Some(Ok(_)) => return Err(Error::ScanInternalError),
                Some(Err(err)) => return Err(err.clone()),
                None => break,
                _ => (),
            }
            if let Some(item) = ti.next() {
                let item = item?;
                if let ScanItem::Token(tok) = item {
                    arg.push(tok.clone());
                }
            }
        }
        // Note code is duplicated here rather than using a closure with an
        // Rc<Cell<usize>>. Perhaps there's a better way.
        match ti.peek() {
            None => return Err(Error::MissingClosingParenMacroCall),
            Some(Ok(ScanItem::Token(Token::Comma))) => {
                let _ = ti.next();
                argc += 1;
                buf.push_front(ScanItem::Arg);
                for tok in arg.iter().rev() {
                    buf.push_front(ScanItem::Token(tok.clone()));
                }
            }
            Some(Ok(ScanItem::Token(Token::RParen))) => {
                let _ = ti.next();
                if arg.len() > 0 || argc > 0 {
                    argc += 1;
                    buf.push_front(ScanItem::Arg);
                    for tok in arg.iter().rev() {
                        buf.push_front(ScanItem::Token(tok.clone()));
                    }
                }
                break;
            }
            Some(Ok(ScanItem::Token(_))) => (),
            Some(Ok(_)) => return Err(Error::ScanInternalError),
            Some(Err(err)) => return Err(err.clone()),
            _ => (),
        }
    }
    Ok(argc)
}

#[cfg(test)]
mod test {
    use super::*;

    fn match_token(item: &ScanItem, tok: Token) -> bool {
        match item {
            ScanItem::Token(otok) => *otok == tok,
            _ => false,
        }
    }

    fn match_arg_barrier(item: &ScanItem) -> bool {
        match item {
            ScanItem::Arg => true,
            _ => false,
        }
    }

    #[test]
    fn no_args() {
        let toks = vec![
            Token::LParen,
            Token::RParen,
        ];
        let mut args = VecDeque::new();
        let ti = toks
            .iter()
            .map(|tok| Ok(ScanItem::Token(tok.clone())));

        assert_eq!(get_args(ti, &mut args), Ok(0));

        assert_eq!(args.len(), 0);
    }

    #[test]
    fn one_arg() {
        let toks = vec![
            Token::LParen,
            Token::Ident("Ciao, mondo".to_string()),
            Token::RParen,
        ];
        let mut args = VecDeque::new();
        let ti = toks
            .iter()
            .map(|tok| Ok(ScanItem::Token(tok.clone())));

        assert_eq!(get_args(ti, &mut args), Ok(1));

        assert_eq!(args.len(), 2);
        assert!(match_token(&args[0], Token::Ident("Ciao, mondo".to_string())));
        assert!(match_arg_barrier(&args[1]));
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
        let mut args = VecDeque::new();
        let ti = toks
            .iter()
            .map(|tok| Ok(ScanItem::Token(tok.clone())));

        assert_eq!(get_args(ti, &mut args), Ok(2));

        assert_eq!(args.len(), 6);
        assert!(match_token(&args[0], Token::LParen));
        assert!(match_token(&args[1], Token::StringLit("something".to_string())));
        assert!(match_token(&args[2], Token::RParen));
        assert!(match_arg_barrier(&args[3]));
        assert!(match_token(&args[4], Token::Ident("abc".to_string())));
        assert!(match_arg_barrier(&args[5]));
    }
}
