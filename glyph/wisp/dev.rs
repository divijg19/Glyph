use anyhow::Result;
use std::path::PathBuf;

#[path = "../sparq/mod.rs"]
mod sparq;

pub fn run_dev(path: &PathBuf) -> Result<()> {
    let src = std::fs::read_to_string(path)?;
    match sparq::compile_source(&src) {
        Ok(bytes) => println!("Compiled OK: {} bytes", bytes.len()),
        Err(e) => println!("Compile error: {}", e),
    }
    Ok(())
}
