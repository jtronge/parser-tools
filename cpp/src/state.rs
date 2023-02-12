use crate::PreprocessorOptions;

pub struct State {
    opts: PreprocessorOptions,
}

impl State {
    pub fn new(path: &str, opts: PreprocessorOptions) -> State {
        State {
            opts,
        }
    }
}
