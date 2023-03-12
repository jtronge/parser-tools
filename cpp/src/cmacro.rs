use ctokens::Token;

pub enum Macro {
    Object(Vec<Token>),
    Function(Vec<String>, Vec<Token>),
}
