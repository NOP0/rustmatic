use crate::{InputNumber, OutputNumber};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// An individual IO device (e.g. Ethercat bus).
///
/// # Note To Implementors
///
/// Reading or writing should never block.
///
/// The [`Device`] should print a concise human-readable description using
/// [`Display`].
pub trait Device<T>: Display {
    /// Notify the caller which inputs and outputs are supported.
    fn register(&self, registrar: &mut dyn DeviceRegistrar);

    fn read(&self, number: InputNumber) -> Result<T, DeviceError>;

    fn write(
        &self,
        number: OutputNumber,
        new_state: T,
    ) -> Result<(), DeviceError>;
}

/// The thing passed to a [`Device`] when registering a device with
/// [`Device::register()`].
pub trait DeviceRegistrar {
    /// Marks a particular input as readable.
    fn input(&mut self, number: InputNumber);
    /// Marks a particular output as writeable.
    fn output(&mut self, number: OutputNumber);
}

#[derive(Debug)]
pub enum DeviceError {
    UnknownNumber,
    Other(Box<dyn Error>),
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DeviceError::UnknownNumber => write!(f, "Unknown Number"),
            DeviceError::Other(ref other) => Display::fmt(other, f),
        }
    }
}

impl Error for DeviceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DeviceError::UnknownNumber => None,
            DeviceError::Other(ref other) => Some(&**other),
        }
    }
}

// We can't implement `From<E> where E: Error` because `DeviceError` implements
// `Error` and we'd overlap with `impl<T> From<T> for T` in std. That makes `?`
// a lot less useful 😞

impl From<Box<dyn Error>> for DeviceError {
    fn from(other: Box<dyn Error>) -> DeviceError { DeviceError::Other(other) }
}
