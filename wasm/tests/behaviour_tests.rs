use anyhow::Context;
use rustmatic_wasm_test::{Compiler, TestCase};

macro_rules! wasm_test {
    ($filename:ident, $( $rest:ident ),*) => {
        wasm_test!($filename);
        wasm_test!($($rest),*);
    };
    ($filename:ident) => {
        #[test]
        fn $filename() {
            let _ = env_logger::try_init();

            let src = include_str!(concat!("data/", stringify!($filename), ".rs"));
            let recipe = include_str!(concat!("data/", stringify!($filename), ".json"));

            let tc = TestCase::parse(stringify!($filename), src, recipe)
                .context("Unable to load the test case").unwrap();
            let compiler = Compiler::default();

            rustmatic_wasm_test::run_test_case(&compiler, &tc).unwrap();
        }
    };
}

wasm_test!(example_program, blinky, set_outputs);
