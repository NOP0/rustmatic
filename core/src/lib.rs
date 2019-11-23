//! Core abstractions and datatypes used by the rustmatic PLC environment.

mod device_manager;

pub use device_manager::{DeviceManager, Devices};

use std::time::Instant;

/// The interface exposed to a [`Process`] so it can interact with the outside
/// world.
///
/// # Note To Implementors
///
/// A [`System`] may be used concurrently by multiple [`Process`]es, so it will
/// need to use some form of interior mutability.
pub trait System {
    /// Access the IO subsystem.
    fn devices(&self) -> &DeviceManager;
    /// Get the current time.
    fn now(&self) -> Instant;
    /// Declare a variable which can be accessed by the outside world.
    fn declare_variable(
        &self,
        name: &str,
        initial_value: Value,
    ) -> VariableIndex;
    /// Get a copy of a variable's current value.
    fn read_variable(&self, index: VariableIndex) -> Option<Value>;
    /// Give a variable a new value.
    fn set_variable(&self, index: VariableIndex, new_value: Value);
}

slotmap::new_key_type! {
    /// An opaque handle that can be used to read from an input.
    pub struct InputNumber;

    /// An opaque handle that can be used to write to an output.
    pub struct OutputNumber;

    /// The handle used to access a variable.
    pub struct VariableIndex;

    /// An opaque handle used to access a [`Device<T>`].
    pub struct DeviceID;
}

/// A single thread of execution.
pub trait Process {
    type Fault;

    fn poll(&mut self, system: &dyn System) -> Transition<Self::Fault>;
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

/// The thing passed to a [`Device<T>`] when registering a device with
/// [`Device<T>::register()`].
pub trait DeviceRegistrar {
    /// Marks a particular input as readable.
    fn input(&mut self, number: InputNumber);
    /// Marks a particular output as writeable.
    fn output(&mut self, number: OutputNumber);
}

/// An individual IO device (e.g. Ethercat bus).
///
/// # Note To Implementors
///
/// If the `index` is out of range, the `Device<T>` *must* fail early with
/// [`DeviceError::UnknownNumber`]. This allows callers to test whether an
/// input is supported by trying a `read()` or `write()` and checking for an
/// error.
pub trait Device<T> {
    /// A human-readable, one-line description of the device.
    fn description(&self) -> &str;

    /// Notify the caller which inputs and outputs are supported.
    fn register(&self, registrar: &mut dyn DeviceRegistrar);

    fn read(&self, number: InputNumber) -> Result<T, DeviceError>;

    fn write(
        &self,
        number: OutputNumber,
        new_state: T,
    ) -> Result<(), DeviceError>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DeviceError {
    UnknownNumber,
}

/// All value types known to the PLC runtime.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    Double(f64),
    String(String),
}
