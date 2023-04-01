//! Simple forth implementation

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
    Err(ExecError::NotImplemented)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_expr() {
        assert_eq!(execute("1 2 add").unwrap(), ExecResult::Integer(3));
    }
}
