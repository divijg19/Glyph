use glyph::sparq::{ast, lexer, parser};

#[test]
fn parse_function_return_number() {
    let src = "fn main(){ return 5 }";
    let toks = lexer::lex(src).expect("lex");
    let prog = parser::parse_program(&toks).expect("parse");
    assert_eq!(prog.functions.len(), 1);
    let f = &prog.functions[0];
    assert_eq!(f.name, "main");
    match &f.body.statements[0] {
        ast::Stmt::Return(expr) => match expr {
            ast::Expr::Number(n) => assert_eq!(*n, 5),
            _ => panic!("expected number"),
        },
    }
}

#[test]
fn parse_arithmetic_precedence() {
    let src = "fn main(){ return 2 + 3 * 4 }";
    let toks = lexer::lex(src).expect("lex");
    let prog = parser::parse_program(&toks).expect("parse");
    let expr = match &prog.functions[0].body.statements[0] {
        ast::Stmt::Return(e) => e,
    };

    // expect 2 + (3 * 4)
    match expr {
        ast::Expr::Binary(left, _op, right) => {
            // left should be number 2
            match &**left {
                ast::Expr::Number(n) => assert_eq!(*n, 2),
                _ => panic!("left not number"),
            }
            // right should be Binary(3 * 4)
            match &**right {
                ast::Expr::Binary(l2, _op2, r2) => {
                    match &**l2 {
                        ast::Expr::Number(n) => assert_eq!(*n, 3),
                        _ => panic!(),
                    }
                    match &**r2 {
                        ast::Expr::Number(n) => assert_eq!(*n, 4),
                        _ => panic!(),
                    }
                }
                _ => panic!("expected nested multiplication"),
            }
        }
        _ => panic!("expected binary expression"),
    }
}

#[test]
fn parse_function_call() {
    let src = "fn add(a,b){ return a+b } fn main(){ return add(2,3) }";
    let toks = lexer::lex(src).expect("lex");
    let prog = parser::parse_program(&toks).expect("parse");
    match &prog.functions[1].body.statements[0] {
        ast::Stmt::Return(ast::Expr::Call(name, args)) => {
            assert_eq!(name, "add");
            assert_eq!(args.len(), 2);
        }
        _ => panic!("expected call"),
    }
}
