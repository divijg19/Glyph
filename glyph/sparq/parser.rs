use std::fmt;

use super::ast::*;
use super::lexer::{Token, TokenKind};

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}:{}", self.message, self.line, self.column)
    }
}

impl std::error::Error for ParseError {}

type PResult<T> = std::result::Result<T, ParseError>;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        // safe fallback to last token (should be EOF)
        self.tokens
            .get(self.pos)
            .unwrap_or_else(|| self.tokens.last().expect("tokens must contain EOF"))
    }

    fn bump(&mut self) {
        if self.pos < self.tokens.len().saturating_sub(1) {
            self.pos += 1;
        }
    }

    fn error_at(&self, msg: &str) -> ParseError {
        let t = self.peek();
        ParseError {
            message: msg.to_string(),
            line: t.line,
            column: t.column,
        }
    }

    fn expect_identifier(&mut self) -> PResult<String> {
        match &self.peek().kind {
            TokenKind::Identifier(name) => {
                let n = name.clone();
                self.bump();
                Ok(n)
            }
            other => Err(ParseError {
                message: format!("expected identifier, got: {:?}", other),
                line: self.peek().line,
                column: self.peek().column,
            }),
        }
    }

    fn expect_token<F>(&mut self, f: F, msg: &str) -> PResult<()>
    where
        F: Fn(&TokenKind) -> bool,
    {
        if f(&self.peek().kind) {
            self.bump();
            Ok(())
        } else {
            Err(self.error_at(msg))
        }
    }

    pub fn parse_program(&mut self) -> PResult<Program> {
        let mut functions = Vec::new();
        while !matches!(self.peek().kind, TokenKind::Eof) {
            functions.push(self.parse_function()?);
        }
        Ok(Program { functions })
    }

    fn parse_function(&mut self) -> PResult<Function> {
        self.expect_token(|k| matches!(k, TokenKind::Fn), "expected 'fn'")?;
        let name = self.expect_identifier()?;

        self.expect_token(|k| matches!(k, TokenKind::LParen), "expected '('")?;

        let mut params = Vec::new();
        if !matches!(self.peek().kind, TokenKind::RParen) {
            // at least one param
            params.push(self.expect_identifier()?);
            while matches!(self.peek().kind, TokenKind::Comma) {
                self.bump();
                params.push(self.expect_identifier()?);
            }
        }

        self.expect_token(|k| matches!(k, TokenKind::RParen), "expected ')'")?;

        let body = self.parse_block()?;

        Ok(Function { name, params, body })
    }

    fn parse_block(&mut self) -> PResult<Block> {
        self.expect_token(|k| matches!(k, TokenKind::LBrace), "expected '{'")?;
        let mut stmts = Vec::new();

        while !matches!(self.peek().kind, TokenKind::RBrace) {
            // only return statements supported
            stmts.push(self.parse_statement()?);
        }

        self.expect_token(|k| matches!(k, TokenKind::RBrace), "expected '}'")?;
        Ok(Block { statements: stmts })
    }

    fn parse_statement(&mut self) -> PResult<Stmt> {
        if matches!(self.peek().kind, TokenKind::Return) {
            self.bump();
            let expr = self.parse_expression()?;
            Ok(Stmt::Return(expr))
        } else {
            Err(self.error_at("unexpected token in block"))
        }
    }

    fn parse_expression(&mut self) -> PResult<Expr> {
        self.parse_precedence(0)
    }

    fn get_op_prec(&self) -> Option<(BinOp, u8)> {
        match &self.peek().kind {
            TokenKind::Plus => Some((BinOp::Add, 1)),
            TokenKind::Minus => Some((BinOp::Sub, 1)),
            TokenKind::Star => Some((BinOp::Mul, 2)),
            TokenKind::Slash => Some((BinOp::Div, 2)),
            _ => None,
        }
    }

    fn parse_precedence(&mut self, min_prec: u8) -> PResult<Expr> {
        let mut left = self.parse_primary()?;

        loop {
            let op_prec = self.get_op_prec();
            if let Some((op, prec)) = op_prec {
                if prec < min_prec {
                    break;
                }
                // consume operator
                self.bump();
                // parse rhs with higher precedence for right-associativity handling
                let right = self.parse_precedence(prec + 1)?;
                left = Expr::Binary(Box::new(left), op, Box::new(right));
                continue;
            }
            break;
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> PResult<Expr> {
        match &self.peek().kind {
            TokenKind::Number(n) => {
                let v = *n;
                self.bump();
                Ok(Expr::Number(v))
            }
            TokenKind::LParen => {
                // grouping
                self.bump();
                let expr = self.parse_expression()?;
                self.expect_token(|k| matches!(k, TokenKind::RParen), "expected ')'")?;
                Ok(expr)
            }
            TokenKind::Identifier(name) => {
                let id = name.clone();
                // lookahead for call
                self.bump();
                if matches!(self.peek().kind, TokenKind::LParen) {
                    // call
                    self.bump();
                    let mut args = Vec::new();
                    if !matches!(self.peek().kind, TokenKind::RParen) {
                        args.push(self.parse_expression()?);
                        while matches!(self.peek().kind, TokenKind::Comma) {
                            self.bump();
                            args.push(self.parse_expression()?);
                        }
                    }
                    self.expect_token(|k| matches!(k, TokenKind::RParen), "expected ')'")?;
                    Ok(Expr::Call(id, args))
                } else {
                    Ok(Expr::Identifier(id))
                }
            }
            other => Err(ParseError {
                message: format!("unexpected primary token: {:?}", other),
                line: self.peek().line,
                column: self.peek().column,
            }),
        }
    }
}

pub fn parse_program(tokens: &Vec<Token>) -> PResult<Program> {
    let mut p = Parser::new(tokens.clone());
    p.parse_program()
}
