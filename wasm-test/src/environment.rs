use crate::Pass;
use anyhow::Error;
use log::{Level, Record};
use rustmatic_wasm::{Error as WasmError, Value};
use std::time::Duration;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TestEnvironment {
    elapsed: Duration,
    inputs: Vec<u8>,
    outputs: Vec<u8>,
    log_messages: Vec<(Level, String)>,
}

impl TestEnvironment {
    pub fn setup(&mut self, pass: &Pass) {
        assert_ne!(pass.delta_time, Duration::new(0, 0));

        self.elapsed += pass.delta_time;
        self.load_inputs(&pass.inputs);
        self.outputs = vec![0; pass.expected_outputs.len()];
        self.log_messages.clear();
    }

    fn load_inputs(&mut self, inputs: &[u8]) {
        self.inputs.clear();
        self.inputs.extend(inputs);
    }

    pub fn compare_outputs(&self, pass: &Pass) -> Result<(), Error> {
        if self.outputs != pass.expected_outputs {
            anyhow::bail!("{:?} != {:?}", self.outputs, pass.expected_outputs);
        }

        Ok(())
    }
}

impl rustmatic_wasm::Environment for TestEnvironment {
    fn elapsed(&self) -> Result<Duration, WasmError> { unimplemented!() }

    fn read_input(
        &self,
        address: usize,
        buffer: &mut [u8],
    ) -> Result<(), WasmError> {
        unimplemented!()
    }

    fn write_output(
        &mut self,
        address: usize,
        buffer: &[u8],
    ) -> Result<(), WasmError> {
        unimplemented!()
    }

    fn log(&mut self, record: &Record<'_>) -> Result<(), WasmError> {
        self.log_messages
            .push((record.level(), record.args().to_string()));
        Ok(())
    }

    fn get_variable(&self, name: &str) -> Result<Value, WasmError> {
        unimplemented!()
    }

    fn set_variable(
        &mut self,
        name: &str,
        value: Value,
    ) -> Result<(), WasmError> {
        unimplemented!()
    }
}
