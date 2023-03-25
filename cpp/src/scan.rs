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

struct ScanBuffer<I> {
    pub stream: ScanStream<I>,
    pub staging: VecDeque<ScanItem>,
}

impl<I> ScanBuffer<I>
where
    I: Iterator<Item=Result<Token>>,
{
    fn new(input: I) -> ScanBuffer<I> {
        ScanBuffer {
            stream: ScanStream {
                input,
                tmp: VecDeque::new(),
            },
            staging: VecDeque::new(),
        }
    }
}

struct ScanStream<I> {
    input: I,
    tmp: VecDeque<ScanItem>,
}

impl<I> ScanStream<I>
where
    I: Iterator<Item=Result<Token>>,
{
    /// Return true if the temporary internal buffer is empty, thus no more
    /// immediate tokens to process for now.
    fn empty(&self) -> bool {
        self.tmp.len() == 0
    }

    /// Push an item on to the stream
    fn push(&mut self, item: ScanItem) {
        self.tmp.push_front(item);
    }
}

impl<I> Iterator for ScanStream<I>
where
    I: Iterator<Item=Result<Token>>,
{
    type Item = Result<ScanItem>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.tmp.pop_front() {
            Some(Ok(item))
        } else {
            self.input.next()
                .map(|tok| Ok(ScanItem::Token(tok?)))
        }
    }
}

// TODO: Maybe there should be a # and ## operator here as well?
enum ScanItem {
    FnMacro(Rc<Macro>),
    Arg,
    Token(Token),
}

/// Determine the concatation operation indices (i.e. `name##arg`).
///
/// There needs to be a better way to encode this initially.
fn determine_concat_args(toks: &[Token]) -> Vec<usize> {
    let mut idx = vec![];
    let mut i = 0;
    for i in 0..toks.len() {
        if i > 0 && i < (toks.len() - 1){
            if let Token::DoubleHash = toks[i] {
                idx.push(i - 1);
                idx.push(i + 1);
            }
        }
    }
    idx
}

fn determine_string_args(toks: &[Token]) -> Vec<usize> {
    vec![]
}

/// Handle a function macro on the stream
fn fn_macro<I>(
    state: &Rc<RefCell<State>>,
    mac: Rc<Macro>,
    sbuf: &mut ScanBuffer<I>,
)
where
    I: Iterator<Item=Result<Token>>,
{
    if let Macro::Function(ref args, ref toks) = *mac {
        // Determine the argument indices in staging
        let mut idx = vec![];
        let mut i = 0;
        while idx.len() < args.len() {
            let start = i;
            loop {
                if let ScanItem::Arg = sbuf.staging[i] {
                    break;
                }
                i += 1;
            }
            idx.push((start, i));
        }
        // Determine the concatation and stringify args
        let concat_args = determine_concat_args(toks);
        let string_args = determine_string_args(toks);
        // Index of the arg being replaced
        let mut repl_arg_idx = 0;
        for tok in toks {
            if let Token::Ident(ident) = tok {
                // If it's an argument, then replace it
                if let Some(j) = args.iter().position(|arg| *arg == *ident) {
                    for k in idx[j].0..idx[j].1 {
                        if let ScanItem::Token(other_tok) = &sbuf.staging[k] {
                            sbuf.stream.push(ScanItem::Token(other_tok.clone()));
                        } else {
                            panic!("Found incorrect value in staging buffer");
                        }
                        repl_arg_idx += 1;
                    }
                } else {
                    sbuf.stream.push(ScanItem::Token(tok.clone()));
                }
            } else {
                sbuf.stream.push(ScanItem::Token(tok.clone()));
            }
        }
        println!("{:?}", idx);
    } else {
        panic!("Invalid macro on scan stream");
    }
}

pub fn scan<'a>(
    state: Rc<RefCell<State>>,
    mut ti: impl Iterator<Item=Result<Token>> + 'a,
    ready: &mut VecDeque<Token>,
) -> Result<()> {
    let mut sbuf = ScanBuffer::new(ti);
    // let mut staging = VecDeque::new();
    // let mut tmp = VecDeque::new();

    loop {
        if let Some(item) = sbuf.stream.next() {
            match item? {
                ScanItem::FnMacro(mac) => fn_macro(&state, mac, &mut sbuf),
                ScanItem::Arg => {
                    // Nothing to do here
                    sbuf.staging.push_back(ScanItem::Arg);
                }
                ScanItem::Token(tok) => {
                    // Process the token
                    if let Some(mac) = state.borrow().find_macro(&tok) {
                        match *mac {
                            Macro::Function(ref args, ref toks) => {
                                // Do function macro replacement
                                let count = get_args(&mut sbuf.stream, &mut sbuf.staging)?;
                                if count != args.len() {
                                    return Err(Error::InvalidMacroArgumentCount);
                                }
                            }
                            Macro::Object(ref toks) => {
                                for tok in toks {
                                    sbuf.stream.push(ScanItem::Token(tok.clone()));
                                }
                            }
                        }
                    } else {
                        // TODO: Should this go on staging or be pushed back onto the stream?
                        sbuf.staging.push_front(ScanItem::Token(tok));
                    }
                }
            }
        } else {
            break;
        }
        if sbuf.stream.empty() {
            // Nothing left to process for now
            break;
        }
    }
    // Add everything left in tmp to the ready buffer
    ready.extend(
        sbuf.staging
            .iter()
            .filter_map(|item| match item {
                ScanItem::FnMacro(_) => None,
                ScanItem::Arg => None,
                ScanItem::Token(tok) => Some(tok.clone()),
            })
    );
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
