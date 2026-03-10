use anyhow::Result;
use std::path::Path;

pub fn run_wasm_file(path: &Path) -> Result<i32> {
    let bytes = std::fs::read(path).map_err(|e| anyhow::anyhow!(format!("failed to read wasm file: {}", e)))?;
    run_wasm_bytes(&bytes)
}

pub fn run_wasm_bytes(bytes: &[u8]) -> Result<i32> {
    let engine = wasmtime::Engine::default();
    let module = wasmtime::Module::new(&engine, bytes)
        .map_err(|e| anyhow::anyhow!(format!("WASM compile error: {}", e)))?;
    let mut store = wasmtime::Store::new(&engine, ());
    let instance = wasmtime::Instance::new(&mut store, &module, &[])
        .map_err(|e| anyhow::anyhow!(format!("WASM instantiate error: {}", e)))?;

    // try to get a typed `main: () -> i32` export
    match instance.get_typed_func::<(), i32>(&mut store, "main") {
        Ok(main) => match main.call(&mut store, ()) {
            Ok(result) => {
                println!("{}", result);
                Ok(result)
            }
            Err(e) => Err(anyhow::anyhow!(format!("Runtime trap during execution: {}", e))),
        },
        Err(_) => Err(anyhow::anyhow!("Runtime error: exported function 'main' not found or has incompatible signature")),
    }
}
