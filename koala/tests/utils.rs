use std::path::PathBuf;

pub fn fixture_path(name: &str) -> String {
    let mut buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    buf.push("tests/fixtures/");
    buf.push(name);
    return buf.display().to_string();
}
