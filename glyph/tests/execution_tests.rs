use glyph::sparq;
use glyph::runtime;

#[test]
fn run_simple_return() {
    let src = "fn main(){ return 42 }";
    let wasm = sparq::compile_source(src).expect("compile");
    let r = runtime::run_wasm_bytes(&wasm).expect("run");
    assert_eq!(r, 42);
}

#[test]
fn run_add_example() {
    let src = std::fs::read_to_string("../examples/basic/add.gly").expect("read");
    let wasm = sparq::compile_source(&src).expect("compile");
    let r = runtime::run_wasm_bytes(&wasm).expect("run");
    assert_eq!(r, 5);
}

#[test]
fn nested_calls() {
    let src = "fn add(a,b){ return a+b } fn square(x){ return x*x } fn main(){ return square(add(2,3)) }";
    let wasm = sparq::compile_source(src).expect("compile");
    let r = runtime::run_wasm_bytes(&wasm).expect("run");
    assert_eq!(r, 25);
}

#[test]
fn deterministic_compilation() {
    let src = std::fs::read_to_string("../examples/basic/add.gly").expect("read");
    let a = sparq::compile_source(&src).expect("compile1");
    let b = sparq::compile_source(&src).expect("compile2");
    assert_eq!(a, b, "WASM outputs differ between compilations");
}

#[test]
fn runtime_missing_main_detected() {
    // build an AST without main by parsing and removing main
    let src = "fn foo(){ return 1 }";
    let toks = glyph::sparq::lexer::lex(src).expect("lex");
    let prog = glyph::sparq::parser::parse_program(&toks).expect("parse");
    // compile without validation by calling codegen directly
    let wasm = glyph::sparq::codegen::compile_program(&prog).expect("codegen");
    let res = runtime::run_wasm_bytes(&wasm);
    assert!(res.is_err());
}
