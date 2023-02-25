use ctokens::Token;

pub enum Macro {
    Object(Vec<Token>),
    Function(Vec<String>, Vec<Token>),
}

/*
pub enum MacroData {
    Object(Vec<Token<P>>),
    Function(Vec<String>, Vec<Token<P>>),
}

pub(crate) struct Macro<P: AsRef<Path>> {
    data: MacroData<P>,
    path: P,
    i: usize,
}
*/
