use crate::{InputNumber, OutputNumber};

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

/// The thing passed to a [`Device<T>`] when registering a device with
/// [`Device<T>::register()`].
pub trait DeviceRegistrar {
    /// Marks a particular input as readable.
    fn input(&mut self, number: InputNumber);
    /// Marks a particular output as writeable.
    fn output(&mut self, number: OutputNumber);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DeviceError {
    UnknownNumber,
}
