use crate::{Device, DeviceError, DeviceID};
use anymap::AnyMap;
use slotmap::DenseSlotMap;
use std::sync::Arc;

/// A collection of [`Device`]s.
pub struct DeviceManager {
    devices: AnyMap,
}

impl DeviceManager {
    pub fn new() -> Self {
        DeviceManager {
            devices: AnyMap::new(),
        }
    }

    /// Get all devices which can read/write values of type `T`.
    pub fn of_type<T: 'static>(&self) -> Option<&Devices<T>> {
        self.devices.get::<Devices<T>>()
    }

    /// Register a new device.
    pub fn register<T: 'static>(
        &mut self,
        device: Arc<dyn Device<T>>,
    ) -> DeviceID {
        let devices = self
            .devices
            .entry::<Devices<T>>()
            .or_insert_with(Devices::<T>::default);

        let id = devices.0.insert(Arc::clone(&device));

        id
    }

    pub fn read<T: 'static>(
        &self,
        device_id: DeviceID,
    ) -> Result<T, DeviceError> {
        let device = self
            .of_type::<T>()
            .and_then(|devices| devices.0.get(device_id))
            .ok_or(DeviceError::UnknownNumber)?;

        device.read()
    }

    pub fn digital_read(
        &self,
        device_id: DeviceID,
    ) -> Result<bool, DeviceError> {
        self.read(device_id)
    }

    pub fn analogue_read(
        &self,
        device_id: DeviceID,
    ) -> Result<f32, DeviceError> {
        self.read(device_id)
    }

    pub fn write<T: 'static>(
        &self,
        device_id: DeviceID,
        new_state: T,
    ) -> Result<(), DeviceError> {
        let device = self
            .of_type::<T>()
            .and_then(|devices| devices.0.get(device_id))
            .ok_or(DeviceError::UnknownNumber)?;

        device.write(new_state)
    }

    pub fn digital_write(
        &self,
        device_id: DeviceID,
        new_state: bool,
    ) -> Result<(), DeviceError> {
        self.write(device_id, new_state)
    }

    pub fn analogue_write(
        &self,
        device_id: DeviceID,
        new_state: f32,
    ) -> Result<(), DeviceError> {
        self.write(device_id, new_state)
    }
}

/// The type returned from [`DeviceManager::of_type()`]. You probably don't want
/// want to use this directly.
#[derive(Clone)]
pub struct Devices<T>(DenseSlotMap<DeviceID, Arc<dyn Device<T>>>);

impl<T> Default for Devices<T> {
    fn default() -> Devices<T> { Devices(DenseSlotMap::with_key()) }
}
