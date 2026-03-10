use logos::Logos;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Fn,
    Return,
    Identifier(String),
    Number(i32),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Plus,
    Minus,
    Star,
    Slash,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}:{}", self.message, self.line, self.column)
    }
}

impl std::error::Error for LexError {}

#[derive(Logos, Debug, PartialEq)]
enum Lexeme {
    #[token("fn")]
    Fn,

    #[token("return")]
    Return,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex("[0-9]+")]
    Number,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,

    #[error]
    #[regex("[ \t\n\r]+", logos::skip)]
    Error,
}

fn compute_line_starts(input: &str) -> Vec<usize> {
    let mut starts = vec![0usize];
    for (i, ch) in input.char_indices() {
        if ch == '\n' {
            starts.push(i + ch.len_utf8());
        }
    }
    starts
}

fn pos_to_line_col(starts: &[usize], pos: usize) -> (usize, usize) {
    // find greatest start <= pos
    let mut line = 0usize;
    for (i, &s) in starts.iter().enumerate() {
        if s <= pos {
            line = i;
        } else {
            break;
        }
    }
    let line_num = line + 1; // 1-based
    let col = pos.saturating_sub(starts[line]) + 1;
    (line_num, col)
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexeme::lexer(input);
    let mut tokens = Vec::new();
    let line_starts = compute_line_starts(input);

    while let Some(tok) = lexer.next() {
        let span = lexer.span();
        let start = span.start;
        let (line, column) = pos_to_line_col(&line_starts, start);

        match tok {
            Lexeme::Fn => tokens.push(Token { kind: TokenKind::Fn, line, column }),
            Lexeme::Return => tokens.push(Token { kind: TokenKind::Return, line, column }),
            Lexeme::Identifier => {
                let slice = lexer.slice();
                tokens.push(Token { kind: TokenKind::Identifier(slice.to_string()), line, column })
            }
            Lexeme::Number => {
                let slice = lexer.slice();
                match slice.parse::<i32>() {
                    Ok(v) => tokens.push(Token { kind: TokenKind::Number(v), line, column }),
                    Err(_) => return Err(LexError { message: format!("invalid number: {}", slice), line, column }),
                }
            }
            Lexeme::LParen => tokens.push(Token { kind: TokenKind::LParen, line, column }),
            Lexeme::RParen => tokens.push(Token { kind: TokenKind::RParen, line, column }),
            Lexeme::LBrace => tokens.push(Token { kind: TokenKind::LBrace, line, column }),
            Lexeme::RBrace => tokens.push(Token { kind: TokenKind::RBrace, line, column }),
            Lexeme::Comma => tokens.push(Token { kind: TokenKind::Comma, line, column }),
            Lexeme::Plus => tokens.push(Token { kind: TokenKind::Plus, line, column }),
            Lexeme::Minus => tokens.push(Token { kind: TokenKind::Minus, line, column }),
            Lexeme::Star => tokens.push(Token { kind: TokenKind::Star, line, column }),
            Lexeme::Slash => tokens.push(Token { kind: TokenKind::Slash, line, column }),
            Lexeme::Error => {
                // report first unexpected character at this span
                let ch = input[start..].chars().next().unwrap_or('\u{FFFD}');
                return Err(LexError { message: format!("Unexpected character '{}'", ch), line, column });
            }
        }
    }

    // EOF position is end of input
    let eof_pos = input.len();
    let (line, column) = pos_to_line_col(&line_starts, eof_pos);
    tokens.push(Token { kind: TokenKind::Eof, line, column });
    Ok(tokens)
}
