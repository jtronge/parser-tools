use std::rc::Rc;
use std::collections::HashMap;
use crate::cmacro::Macro;

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
}
