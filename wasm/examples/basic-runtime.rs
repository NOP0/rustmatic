use rustmatic_wasm::{InMemory, Program};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let wasm_file = match env::args().skip(1).next() {
        Some(filename) => filename,
        None => panic!("Usage: basic-runtime <wasm-file>"),
    };

    let mut env = InMemory::default();

    let wasm = std::fs::read(&wasm_file)?;
    let mut program = Program::load("basic-runtime", &wasm)?;

    loop {
        program.poll(&mut env)?;
    }
}
