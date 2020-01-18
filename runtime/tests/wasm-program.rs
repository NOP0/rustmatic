use rustmatic_wasm::{Program};
use rustmatic_wasm_test::{Compiler};
use rustmatic_runtime::{Runtime, WasmProcess};
use std::env;
use std::fs;



#[test]
fn wasm_poll(){

    let compiler = Compiler::default();

    let source = fs::read_to_string("./tests/data/example_program.rs")
        .expect("Something went wrong reading the file");

    let wasm: Program = compiler.instantiate("test_program", &source).unwrap();

    let mut runtime = Runtime::new();

    let wasm_process = WasmProcess::new(wasm);

    runtime.add_process(wasm_process);

    runtime.init().expect("Could not init runtime");

    let _ = runtime.poll();
    }