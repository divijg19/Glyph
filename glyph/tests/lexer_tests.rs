use glyph::sparq::lexer;

#[test]
fn tokenize_function_return() {
    let src = "fn main(){ return 5 }";
    let toks = lexer::lex(src).expect("lex failed");
    let kinds: Vec<_> = toks
        .iter()
        .map(|t| match &t.kind {
            lexer::TokenKind::Fn => "Fn",
            lexer::TokenKind::Identifier(_) => "Identifier",
            lexer::TokenKind::LParen => "LParen",
            lexer::TokenKind::RParen => "RParen",
            lexer::TokenKind::LBrace => "LBrace",
            lexer::TokenKind::Return => "Return",
            lexer::TokenKind::Number(_) => "Number",
            lexer::TokenKind::RBrace => "RBrace",
            lexer::TokenKind::Eof => "Eof",
            _ => "Other",
        })
        .collect();

    assert_eq!(kinds[0], "Fn");
    assert_eq!(kinds[1], "Identifier");
    assert_eq!(kinds[2], "LParen");
}

#[test]
fn tokenize_arithmetic() {
    let src = "2 + 3 * 4";
    let toks = lexer::lex(src).expect("lex failed");
    let seq: Vec<_> = toks
        .iter()
        .map(|t| match &t.kind {
            lexer::TokenKind::Number(_) => "Number",
            lexer::TokenKind::Plus => "Plus",
            lexer::TokenKind::Star => "Star",
            _ => "Other",
        })
        .filter(|s| *s != "Other")
        .collect();

    assert_eq!(seq, vec!["Number", "Plus", "Number", "Star", "Number"]);
}

#[test]
fn lexer_reports_unexpected_char() {
    let src = "@";
    let res = lexer::lex(src);
    assert!(res.is_err());
}
