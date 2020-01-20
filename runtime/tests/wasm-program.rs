use rustmatic_runtime::{Runtime, WasmProcess};
use rustmatic_wasm::Program;
use rustmatic_wasm_test::Compiler;
use std::{cell::RefCell, env, fs, io::Write, rc::Rc};

#[test]
fn wasm_poll() {

    // Mocked out log destination for testing purposes.
    struct VecLogger {
        inner: Rc<RefCell<Vec<u8>>>,
    };

    impl VecLogger {
        pub fn new(vec: Rc<RefCell<Vec<u8>>>) -> VecLogger {
            VecLogger { inner: vec }
        }

        pub fn vec8_as_string(vec: Rc<RefCell<Vec<u8>>>) -> String {
            std::str::from_utf8(&vec.borrow()).unwrap().to_string()
        }
    }

    impl Write for VecLogger {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            (self.inner.borrow_mut()).extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> { unimplemented!() }
    }

    let compiler = Compiler::default();

    let source = fs::read_to_string("./tests/data/example_program.rs")
        .expect("Something went wrong reading the file");

    let wasm: Program = compiler.instantiate("test_program", &source).unwrap();

    let mut runtime = Runtime::new();

    let vec = Rc::new(RefCell::new(Vec::new()));

    let vec_logger = VecLogger::new(vec.clone());

    runtime.logger = Box::new(vec_logger); // Replace stdout logger with vec_logger

    let wasm_process = WasmProcess::new(wasm);

    runtime.add_process(wasm_process);

    runtime.init().expect("Could not init runtime");

    let _ = runtime.poll();

    assert_eq!(VecLogger::vec8_as_string(vec), "Polling");
}
