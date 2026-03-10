pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod validate;

use anyhow::Result;

pub fn compile_source(source: &str) -> Result<Vec<u8>> {
    let tokens = lexer::lex(source).map_err(|e| anyhow::anyhow!(format!("Lex error: {}", e)))?;
    let program = parser::parse_program(&tokens)
        .map_err(|e| anyhow::anyhow!(format!("Parse error: {}", e)))?;
    validate::validate_program(&program)
        .map_err(|e| anyhow::anyhow!(format!("Validation error: {}", e)))?;
    let wasm = codegen::compile_program(&program)?;
    Ok(wasm)
}
