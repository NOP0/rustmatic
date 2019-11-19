//! Core abstractions and datatypes used by the rustmatic PLC environment.

use std::time::Instant;

/// The interface exposed to a [`Process`] so it can interact with the outside
/// world.
///
/// # Note To Implementors
///
/// A [`System`] may be used concurrently by multiple [`Process`]es, so it will
/// need to use some form of interior mutability.
pub trait System {
    /// Access the machine's digital IOs.
    fn digital_io(&self) -> &dyn InputOutput<bool>;
    /// The system clock.
    fn clock(&self) -> &dyn Clock;
    /// Variables which can be exposed across [`Process`]es.
    fn variables(&self) -> &dyn Variables;
}

/// Work with globally exposed variables.
pub trait Variables {
    /// Declare a variable associated to this process which can be accessed by
    /// the outside world.
    fn declare(
        &self,
        name: &str,
        initial_value: Value,
    ) -> Result<VariableIndex, VariableError>;
    /// Get a copy of a variable's current value.
    fn read(&self, index: VariableIndex) -> Option<Value>;
    /// Give a variable a new value.
    fn set(&self, index: VariableIndex, new_value: Value);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VariableError {
    DuplicateVariable { previous_definition: VariableIndex },
}

/// Something which can do IO.
pub trait InputOutput<T> {
    /// Read the state of a single input.
    fn read(&self, _number: InputNumber) -> Result<T, InputOutputError>;

    /// Set the state of a single output.
    fn write(
        &self,
        _number: OutputNumber,
        _value: T,
    ) -> Result<(), InputOutputError>;
}

pub trait Clock {
    /// Get the current time.
    fn now(&self) -> Instant;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputOutputError {
    Unsupported,
    UnknownNumber,
}

slotmap::new_key_type! {
    /// An opaque handle that can be used to read from an input.
    pub struct InputNumber;

    /// An opaque handle that can be used to write to an output.
    pub struct OutputNumber;

    /// The handle used to access a variable.
    pub struct VariableIndex;

    /// A handle used when referring to processes.
    pub struct ProcessID;
}

/// A single thread of execution.
pub trait Process {
    type Fault;

    fn poll(&mut self, system: &dyn System) -> Transition<Self::Fault>;

    /// A callback fired before a process is started, allowing it to do any
    /// necessary setup (e.g. variable registration or IO discovery).
    fn before_start(&mut self, _system: &dyn System) {}
}

/// What should we do after polling a [`Process`]?
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Transition<F> {
    /// The [`Process`] has completed successfully.
    Completed,
    /// It encountered a runtime error and stopped prematurely.
    Fault(F),
    /// The [`Process`] is still running.
    StillRunning,
}

/// All value types known to the PLC runtime.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    Double(f64),
    String(String),
}
