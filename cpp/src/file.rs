use std::path::Path;

pub struct File<P: AsRef<Path>> {
    path: P,
    data: String,
}
