use std::process::Command;
use std::env;

#[test]
fn cli_invalid_extension() {
    let exe = env::var("CARGO_BIN_EXE_glyph").expect("CARGO_BIN_EXE_glyph not set");
    let out = Command::new(&exe)
        .arg("run")
        .arg("file.txt")
        .output()
        .expect("failed to spawn");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains(".gly"));
}

#[test]
fn cli_missing_file() {
    let exe = env::var("CARGO_BIN_EXE_glyph").expect("CARGO_BIN_EXE_glyph not set");
    let out = Command::new(&exe)
        .arg("run")
        .arg("missing.gly")
        .output()
        .expect("failed to spawn");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("not found") || stderr.contains("source file not found"));
}
