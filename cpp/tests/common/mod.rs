/// Return the full path of a resource.
pub fn resource(fname: &str) -> String {
    let mut path = concat!(env!("CARGO_MANIFEST_DIR"), "/resource").to_string();
    path.push_str("/");
    path.push_str(fname);
    path
}
