use glyph::sparq::{lexer, parser, validate};

#[test]
fn missing_main_fails() {
    let src = "fn add(a,b){ return a+b }";
    let toks = lexer::lex(src).expect("lex");
    let prog = parser::parse_program(&toks).expect("parse");
    let res = validate::validate_program(&prog);
    assert!(res.is_err());
}

#[test]
fn unknown_function_fails() {
    let src = "fn main(){ return foo(2) }";
    let toks = lexer::lex(src).expect("lex");
    let prog = parser::parse_program(&toks).expect("parse");
    let res = validate::validate_program(&prog);
    assert!(res.is_err());
}

#[test]
fn arg_count_mismatch_fails() {
    let src = "fn add(a,b){ return a+b } fn main(){ return add(1) }";
    let toks = lexer::lex(src).expect("lex");
    let prog = parser::parse_program(&toks).expect("parse");
    let res = validate::validate_program(&prog);
    assert!(res.is_err());
}

#[test]
fn duplicate_function_names_fails() {
    let src = "fn add(a,b){ return a+b } fn add(a,b){ return a+b }";
    let toks = lexer::lex(src).expect("lex");
    let prog = parser::parse_program(&toks).expect("parse");
    let res = validate::validate_program(&prog);
    assert!(res.is_err());
}
