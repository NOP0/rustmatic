use log::Record;
use std::{
    collections::HashMap,
    panic,
    panic::AssertUnwindSafe,
    ptr,
    time::{Duration, Instant},
};
use wasmer_runtime::{
    error::{CallError, Error as WasmerError},
    Array, Ctx, Instance, WasmPtr,
};

pub trait Environment {
    fn elapsed(&self) -> Result<Duration, Error>;

    fn read_input(
        &self,
        address: usize,
        buffer: &mut [u8],
    ) -> Result<(), Error>;

    fn write_output(
        &mut self,
        address: usize,
        buffer: &[u8],
    ) -> Result<(), Error>;

    fn log(&self, record: &Record) -> Result<(), Error>;

    fn get_variable(&self, name: &str) -> Result<Value, Error>;

    fn set_variable(&mut self, name: &str, value: Value) -> Result<(), Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Tried to access an out-of-bounds address")]
    AddressOutOfBounds,
    #[error("Unknown variable")]
    UnknownVariable,
    #[error("Incorrect variable type")]
    BadVariableType,
    #[error("An error occurred while calling a WASM function")]
    Wasm(#[source] CallError),
    #[error("A custom error occurred")]
    Other(#[source] Box<dyn std::error::Error>),
}

impl From<CallError> for Error {
    fn from(other: CallError) -> Error { Error::Wasm(other) }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Integer(i32),
    Float(f64),
}

impl Value {
    pub fn kind(&self) -> Type {
        match *self {
            Value::Bool(_) => Type::Bool,
            Value::Integer(_) => Type::Integer,
            Value::Float(_) => Type::Float,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Bool,
    Integer,
    Float,
}

/// A user-provided program loaded into memory.
pub struct Program {
    instance: wasmer_runtime::Instance,
}

impl Program {
    pub fn load(wasm: &[u8]) -> Result<Self, WasmerError> {
        let imports = wasmer_runtime::imports! {
            "env" => {
                "print" => wasmer_runtime::func!(print),
            },
        };
        let instance = wasmer_runtime::instantiate(wasm, &imports)?;

        Ok(Program { instance })
    }

    pub fn poll(&mut self) -> Result<(), WasmerError> {
        self.instance.call("poll", &[])?;

        Ok(())
    }
}

/// Print the provided message to the screen.
///
/// Returns `-1` if the operation failed.
fn print(ctx: &mut Ctx, msg: WasmPtr<u8, Array>, length: u32) -> i32 {
    match msg.get_utf8_string(ctx.memory(0), length) {
        Some(msg) => {
            print!("{}", msg);
            0
        },
        None => -1,
    }
}

pub struct InMemory {
    inputs: [u8; 128],
    outputs: [u8; 128],
    variables: HashMap<String, Value>,
    started: Instant,
}

impl Default for InMemory {
    fn default() -> InMemory {
        InMemory {
            inputs: [0; 128],
            outputs: [0; 128],
            variables: HashMap::default(),
            started: Instant::now(),
        }
    }
}

impl Environment for InMemory {
    fn elapsed(&self) -> Result<Duration, Error> { Ok(self.started.elapsed()) }

    fn read_input(
        &self,
        address: usize,
        buffer: &mut [u8],
    ) -> Result<(), Error> {
        let inputs = self
            .inputs
            .get(address..address + buffer.len())
            .ok_or(Error::AddressOutOfBounds)?;

        buffer.copy_from_slice(inputs);
        Ok(())
    }

    fn write_output(
        &mut self,
        address: usize,
        buffer: &[u8],
    ) -> Result<(), Error> {
        let dest = self
            .outputs
            .get_mut(address..address + buffer.len())
            .ok_or(Error::AddressOutOfBounds)?;

        dest.copy_from_slice(buffer);
        Ok(())
    }

    fn log(&self, record: &Record) -> Result<(), Error> {
        log::logger().log(record);
        Ok(())
    }

    fn get_variable(&self, name: &str) -> Result<Value, Error> {
        self.variables
            .get(name)
            .copied()
            .ok_or(Error::UnknownVariable)
    }

    fn set_variable(&mut self, name: &str, value: Value) -> Result<(), Error> {
        use std::collections::hash_map::Entry;

        match self.variables.entry(name.to_string()) {
            Entry::Vacant(entry) => {
                // the variable doesn't exist, we can just insert it
                entry.insert(value);
            },
            Entry::Occupied(mut entry) => {
                // make sure we aren't trying to do something like overwriting a
                // bool with a float
                if entry.get().kind() != value.kind() {
                    return Err(Error::BadVariableType);
                }

                entry.insert(value);
            },
        }

        Ok(())
    }
}
