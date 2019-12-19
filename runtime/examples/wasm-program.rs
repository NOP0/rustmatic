use rustmatic_wasm::{Program};
use rustmatic_runtime::{Runtime, WasmProcess};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let wasm_file = match env::args().skip(1).next() {
        Some(filename) => filename,
        None => panic!("Usage: basic-runtime <wasm-file>"),
    };

    let wasm = std::fs::read(&wasm_file)?;
    let program = Program::load("basic-runtime", &wasm)?;

    let mut runtime = Runtime::new();

    let wasm_process = WasmProcess::new(program);

    runtime.add_process(wasm_process);

    runtime.init().expect("Could not init runtime");

    while runtime.iter_processes().count() > 0 {
        let _ = runtime.poll();
    }
    Ok(())
}
