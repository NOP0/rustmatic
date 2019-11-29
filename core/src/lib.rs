//! Core abstractions and datatypes used by the rustmatic PLC environment.

mod device;
mod device_manager;
mod process_image;

pub use crate::{
    device::{Device, DeviceError, DeviceRegistrar},
    device_manager::{DeviceManager, Devices},
    process_image::{ProcessImage, InputHandle}
};

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

    fn process_image(&self) -> &ProcessImage;
}

slotmap::new_key_type! {
    /// An opaque handle that can be used to read from an input.
    pub struct InputNumber;

    /// An opaque handle that can be used to write to an output.
    pub struct OutputNumber;

    /// The handle used to access a variable.
    pub struct VariableIndex;

    /// An opaque handle used to access a [`Device`].
    pub struct DeviceID;
}

/// A single thread of execution.
pub trait Process {
    type Fault;

    fn poll(
        &mut self,
        system: &dyn System,
    ) -> Transition<Self::Fault>;

    fn init(&mut self,
            system: &dyn System,
        ) -> Result<(), Self::Fault>;
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
