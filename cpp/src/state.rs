use std::rc::Rc;
use std::collections::HashMap;
use crate::cmacro::Macro;
use ctokens::Token;

pub struct State {
    pub defines: HashMap<String, Rc<Macro>>,
    pub files: Vec<String>,
}

impl State {
    /// Create the initial preprocessor state from the given path.
    pub fn new(path: &str) -> State {
        State {
            defines: HashMap::new(),
            files: vec![path.to_string()],
        }
    }

    pub fn find_macro(&self, tok: &Token) -> Option<Rc<Macro>> {
        if let Token::Ident(ref name) = tok {
            if let Some(mac) = self.defines.get(name) {
                Some(Rc::clone(mac))
            } else {
                None
            }
        } else {
            None
        }
    }
}
