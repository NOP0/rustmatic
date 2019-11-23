use anymap::AnyMap;
use rustmatic_core::{
    Device, DeviceError, DeviceRegistrar, InputNumber, OutputNumber,
};
use slotmap::DenseSlotMap;
use std::{any::TypeId, collections::HashMap, sync::Arc};

pub struct DeviceManager {
    devices: AnyMap,
    lookup_table: LookupTable,
}

slotmap::new_key_type! {
    pub struct DeviceID;
}

pub type Devices<T> = DenseSlotMap<DeviceID, Arc<dyn Device<T>>>;

impl DeviceManager {
    pub fn new() -> Self {
        DeviceManager {
            devices: AnyMap::new(),
            lookup_table: LookupTable::default(),
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

        let id = devices.insert(Arc::clone(&device));
        self.lookup_table.register(id, &*device);

        id
    }

    pub fn read<T: 'static>(
        &self,
        number: InputNumber,
    ) -> Result<T, DeviceError> {
        let device_id = self
            .lookup_table
            .find_input_device::<T>(number)
            .ok_or(DeviceError::UnknownNumber)?;

        let device = self
            .of_type::<T>()
            .and_then(|devices| devices.get(device_id))
            .ok_or(DeviceError::UnknownNumber)?;

        device.read(number)
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

    pub fn write<T: 'static>(
        &self,
        number: OutputNumber,
        new_state: T,
    ) -> Result<(), DeviceError> {
        let device_id = self
            .lookup_table
            .find_output_device::<T>(number)
            .ok_or(DeviceError::UnknownNumber)?;

        let device = self
            .of_type::<T>()
            .and_then(|devices| devices.get(device_id))
            .ok_or(DeviceError::UnknownNumber)?;

        device.write(number, new_state)
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

/// A lookup table which lets you look the [`Device<T>`] which can read an
/// input (or output) based on the [`InputNumber`] and type being read/written
/// (via [`TypeId`]).
#[derive(Debug, Default, Clone, PartialEq)]
struct LookupTable {
    // FIXME: Do we actually care about TypeId here? Or could we just record
    // the input/output number and then let the lookup into `Devices<T>`
    // error out?
    inputs: HashMap<(InputNumber, TypeId), DeviceID>,
    outputs: HashMap<(OutputNumber, TypeId), DeviceID>,
}

impl LookupTable {
    fn find_input_device<T: 'static>(
        &self,
        number: InputNumber,
    ) -> Option<DeviceID> {
        let key = (number, TypeId::of::<T>());
        self.inputs.get(&key).copied()
    }

    fn find_output_device<T: 'static>(
        &self,
        number: OutputNumber,
    ) -> Option<DeviceID> {
        let key = (number, TypeId::of::<T>());
        self.outputs.get(&key).copied()
    }

    /// Updates the internal bookkeeping used to track which inputs/outputs a
    /// particular [`Device<T>`] can read from or write to.
    fn register<D, T>(&mut self, id: DeviceID, device: &D)
    where
        D: Device<T> + ?Sized,
        T: 'static,
    {
        self.forget_device(id);

        let mut registrar = Registrar {
            id,
            type_id: TypeId::of::<T>(),
            lookup_table: self,
        };
        device.register(&mut registrar);
    }

    fn forget_device(&mut self, id: DeviceID) {
        // FIXME: Looping over every known input and output seems a bit
        // expensive...
        self.inputs.retain(|_key, device_id| *device_id != id);
        self.outputs.retain(|_key, device_id| *device_id != id);
    }
}

/// A temporary struct used so we can remember the [`DeviceID`] and [`TypeId`]
/// when registering inputs and outputs. Essentially a fancy closure.
struct Registrar<'a> {
    id: DeviceID,
    type_id: TypeId,
    lookup_table: &'a mut LookupTable,
}

impl<'a> DeviceRegistrar for Registrar<'a> {
    fn input(&mut self, number: InputNumber) {
        self.lookup_table
            .inputs
            .insert((number, self.type_id), self.id);
    }

    fn output(&mut self, number: OutputNumber) {
        self.lookup_table
            .outputs
            .insert((number, self.type_id), self.id);
    }
}
