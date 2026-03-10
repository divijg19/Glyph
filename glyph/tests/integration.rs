use glyph::{runtime, sparq};
use std::path::PathBuf;

fn repo_examples_path(rel: &str) -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    PathBuf::from(manifest_dir).join("..").join(rel)
}

#[test]
fn compile_and_run_add() {
    let path = repo_examples_path("examples/basic/add.gly");
    let src = std::fs::read_to_string(&path).expect("read example");
    let wasm = sparq::compile_source(&src).expect("compile failed");
    let result = runtime::run_wasm_bytes(&wasm).expect("runtime failed");
    assert_eq!(result, 5);
}

#[test]
fn compile_and_run_square() {
    let path = repo_examples_path("examples/basic/square.gly");
    let src = std::fs::read_to_string(&path).expect("read example");
    let wasm = sparq::compile_source(&src).expect("compile failed");
    let result = runtime::run_wasm_bytes(&wasm).expect("runtime failed");
    assert_eq!(result, 25);
}

#[test]
fn compile_and_run_arithmetic() {
    let path = repo_examples_path("examples/basic/arithmetic.gly");
    let src = std::fs::read_to_string(&path).expect("read example");
    let wasm = sparq::compile_source(&src).expect("compile failed");
    let result = runtime::run_wasm_bytes(&wasm).expect("runtime failed");
    assert_eq!(result, 20);
}
