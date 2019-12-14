use anyhow::{Context, Error};
use rustmatic_wasm_test::{Compiler, TestCase};

macro_rules! wasm_test {
    ($filename:ident, $( $rest:ident ),*) => {
        wasm_test!($filename);
        wasm_test!($($rest),*);
    };
    ($filename:ident) => {
        #[test]
        fn $filename() -> Result<(), Error> {
            let src = include_str!(concat!("data/", stringify!($filename), ".rs"));
            let recipe = include_str!(concat!("data/", stringify!($filename), ".json"));

            let tc = TestCase::parse(stringify!($filename), src, recipe)
                .context("Unable to load the test case")?;
            let compiler = Compiler::default();

            rustmatic_wasm_test::run_test_case(&compiler, &tc)
        }
    };
}

wasm_test!(example_program);
