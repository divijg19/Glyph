use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[path = "../runtime/execute.rs"]
mod runtime;
#[path = "../sparq/mod.rs"]
mod sparq;
#[path = "../wisp/dev.rs"]
mod wisp;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Glyph v0.0.1 - minimal compiler proof-of-concept"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build { file: PathBuf },
    Run { file: PathBuf },
    Dev { file: PathBuf },
}

fn build_file(path: &PathBuf) -> Result<PathBuf> {
    let src = fs::read_to_string(path).map_err(|e| anyhow::anyhow!(format!("failed to read source file: {}", e)))?;
    let wasm = sparq::compile_source(&src)?;
    let mut out = path.clone();
    out.set_extension("wasm");
    fs::write(&out, &wasm)?;
    println!("Wrote {} bytes to {}", wasm.len(), out.display());
    Ok(out)
}

fn run_file(path: &PathBuf) -> Result<()> {
    let out = build_file(path)?;
    runtime::run_wasm_file(&out)?;
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build { file } => {
            // check extension and existence
            if file.extension().and_then(|s| s.to_str()) != Some("gly") {
                bail!("Glyph source files must use the .gly extension")
            }
            if !file.exists() {
                bail!(format!("source file not found: {}", file.display()))
            }
            let _ = build_file(&file)?;
        }
        Commands::Run { file } => {
            if file.extension().and_then(|s| s.to_str()) != Some("gly") {
                bail!("Glyph source files must use the .gly extension")
            }
            if !file.exists() {
                bail!(format!("source file not found: {}", file.display()))
            }
            run_file(&file)?;
        }
        Commands::Dev { file } => {
            if file.extension().and_then(|s| s.to_str()) != Some("gly") {
                bail!("Glyph source files must use the .gly extension")
            }
            if !file.exists() {
                bail!(format!("source file not found: {}", file.display()))
            }
            wisp::run_dev(&file)?;
        }
    }
    Ok(())
}
