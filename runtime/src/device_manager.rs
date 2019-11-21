use anymap::AnyMap;
use rustmatic_core::{Device, DeviceError, InputNumber, OutputNumber};
use std::sync::Arc;

pub struct DeviceManager {
    devices: AnyMap,
}

type Devices<T> = Vec<Arc<dyn Device<T>>>;

impl DeviceManager {
    pub fn new() -> Self {
        DeviceManager {
            devices: AnyMap::new(),
        }
    }

    /// Get all devices which can read/write values of type `T`.
    pub fn of_type<T: 'static>(&self) -> &[Arc<dyn Device<T>>] {
        self.devices
            .get::<Devices<T>>()
            .map(|devices| devices.as_slice())
            .unwrap_or(&[])
    }

    /// Register a new device.
    pub fn register<T: 'static>(&mut self, device: Arc<dyn Device<T>>) {
        self.devices
            .entry::<Devices<T>>()
            .or_insert_with(Devices::<T>::default)
            .push(device);
    }

    pub fn read<T: 'static>(
        &self,
        index: InputNumber,
    ) -> Result<T, DeviceError> {
        for device in self.of_type::<T>() {
            match device.read(index) {
                Err(DeviceError::UnknownNumber) => continue,
                other => return other,
            }
        }

        Err(DeviceError::UnknownNumber)
    }

    pub fn digital_read(
        &self,
        index: InputNumber,
    ) -> Result<bool, DeviceError> {
        self.read(index)
    }

    pub fn analogue_read(
        &self,
        index: InputNumber,
    ) -> Result<f32, DeviceError> {
        self.read(index)
    }

    pub fn write<T: Copy + 'static>(
        &self,
        index: OutputNumber,
        new_state: T,
    ) -> Result<(), DeviceError> {
        for device in self.of_type::<T>() {
            match device.write(index, new_state) {
                Err(DeviceError::UnknownNumber) => continue,
                other => return other,
            }
        }

        Err(DeviceError::UnknownNumber)
    }

    pub fn digital_write(
        &self,
        index: OutputNumber,
        new_state: bool,
    ) -> Result<(), DeviceError> {
        self.write(index, new_state)
    }

    pub fn analogue_write(
        &self,
        index: OutputNumber,
        new_state: f32,
    ) -> Result<(), DeviceError> {
        self.write(index, new_state)
    }
}
