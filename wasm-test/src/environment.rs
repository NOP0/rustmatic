use crate::Pass;
use anyhow::Error;
use log::{Level, Record};
use rustmatic_wasm::{Error as WasmError, Value};
use std::{collections::HashMap, time::Duration};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TestEnvironment {
    pub elapsed: Duration,
    pub inputs: Vec<u8>,
    pub outputs: Vec<u8>,
    pub log_messages: Vec<(Level, String)>,
    pub variables: HashMap<String, Value>,
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

    pub fn compare(&self, pass: &Pass) -> Result<(), Error> {
        if self.outputs != pass.expected_outputs {
            anyhow::bail!("{:?} != {:?}", self.outputs, pass.expected_outputs);
        }

        for msg in &pass.expected_log_messages {
            if !self
                .log_messages
                .iter()
                .any(|(_, logged)| logged.contains(msg))
            {
                anyhow::bail!("Expected log message \"{}\"", msg);
            }
        }

        Ok(())
    }
}

impl rustmatic_wasm::Environment for TestEnvironment {
    fn elapsed(&self) -> Result<Duration, WasmError> { Ok(self.elapsed) }

    fn read_input(
        &self,
        address: usize,
        buffer: &mut [u8],
    ) -> Result<(), WasmError> {
        log::debug!("Reading {} bytes from input {:#x}", buffer.len(), address);

        let src = self
            .inputs
            .get(address..address + buffer.len())
            .ok_or(WasmError::AddressOutOfBounds)?;
        buffer.copy_from_slice(src);
        log::trace!("Input {:#x} = {:?}", address, src);

        Ok(())
    }

    fn write_output(
        &mut self,
        address: usize,
        buffer: &[u8],
    ) -> Result<(), WasmError> {
        log::debug!("Writing {} bytes to output {:#x}", buffer.len(), address);

        let dest = self
            .inputs
            .get_mut(address..address + buffer.len())
            .ok_or(WasmError::AddressOutOfBounds)?;
        dest.copy_from_slice(buffer);
        log::trace!("Output {:#x} = {:?}", address, dest);

        Ok(())
    }

    fn log(&mut self, record: &Record<'_>) -> Result<(), WasmError> {
        log::logger().log(record);
        log::trace!("Logging {:?}", record);

        self.log_messages
            .push((record.level(), record.args().to_string()));
        Ok(())
    }

    fn get_variable(&self, name: &str) -> Result<Value, WasmError> {
        log::debug!("Getting \"{}\"", name);

        self.variables
            .get(name)
            .copied()
            .ok_or(WasmError::UnknownVariable)
    }

    fn set_variable(
        &mut self,
        name: &str,
        value: Value,
    ) -> Result<(), WasmError> {
        use std::collections::hash_map::Entry;

        match self.variables.entry(name.to_string()) {
            Entry::Vacant(vacant) => {
                log::debug!("Declaring \"{}\" = {:?}", name, value);
                vacant.insert(value);
            },
            Entry::Occupied(mut occupied) => {
                log::debug!(
                    "Overwriting \"{}\" from {:?} to {:?}",
                    name,
                    occupied.get(),
                    value
                );

                if occupied.get().kind() == value.kind() {
                    occupied.insert(value);
                } else {
                    return Err(WasmError::BadVariableType);
                }
            },
        }

        Ok(())
    }
}
