use glyph::sparq::{lexer, parser};

#[test]
fn lexer_tokenize_fn_and_arith() {
    let src = "fn add(a, b) { return a + b }";
    let tokens = lexer::lex(src).expect("lex failed");
    // Expect sequence: Fn, Identifier(add), LParen, Identifier(a), Comma, Identifier(b), RParen, LBrace, Return, Identifier(a), Plus, Identifier(b), RBrace, Eof
    assert!(matches!(tokens[0].kind, lexer::TokenKind::Fn));
    assert!(matches!(tokens[1].kind, lexer::TokenKind::Identifier(_)));
}

#[test]
fn parser_parse_simple_function() {
    let src = "fn main() { return 42 }";
    let tokens = lexer::lex(src).expect("lex failed");
    let program = parser::parse_program(&tokens).expect("parse failed");
    assert_eq!(program.functions.len(), 1);
}
