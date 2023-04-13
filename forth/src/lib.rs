//! Simple forth implementation
use std::str::FromStr;

mod parse;
use parse::{Token, TokenStream};

#[derive(Debug, PartialEq)]
pub enum ExecResult {
    Integer(i64),
}

#[derive(Debug, PartialEq)]
pub enum ExecError {
    NotImplemented,
}

pub type Result<T> = std::result::Result<T, ExecError>;

pub fn execute(code: &str) -> Result<ExecResult> {
    let toks: Vec<Token> = TokenStream::new(code).map(|tok| tok.unwrap()).collect();
    let mut stack = vec![];
    for tok in toks {
        match tok {
            Token::Ident(ref ident) => {
                match &ident[..] {
                    "add" => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(a + b);
                    }
                    "mult" => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(a * b);
                    }
                    _ => return Err(ExecError::NotImplemented),
                }
            }
            Token::Number(ref num) => {
                stack.push(i64::from_str(num).unwrap());
            }
        }
    }
    // Return the top of the stack
    Ok(ExecResult::Integer(stack[0]))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_expr() {
        assert_eq!(execute("1 2 add").unwrap(), ExecResult::Integer(3));
    }

    #[test]
    fn mult_expr() {
        assert_eq!(execute("3 2 mult").unwrap(), ExecResult::Integer(6));
    }
}
