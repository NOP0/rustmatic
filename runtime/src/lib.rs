use rustmatic_core::{Device, InputNumber, OutputNumber, Process, Transition};
use slotmap::DenseSlotMap;
use std::time::Instant;

slotmap::new_key_type! {
    pub struct DeviceIndex;
    pub struct ProcessIndex;
}

type Devices = DenseSlotMap<DeviceIndex, Box<dyn Device>>;
type Processes = DenseSlotMap<ProcessIndex, Box<dyn Process<Fault = Fault>>>;

/// The PLC runtime.
pub struct Runtime {
    pub(crate) devices: Devices,
    pub(crate) processes: Processes,
}

impl Runtime {
    /// Create an empty [`Runtime`].
    pub fn new() -> Self {
        Runtime {
            devices: Devices::with_key(),
            processes: Processes::with_key(),
        }
    }

    /// Get an iterator over all known devices.
    pub fn iter_devices<'this>(&'this self) -> impl Iterator<Item = &'this dyn Device> + 'this {
        self.devices.iter().map(|(_key, boxed)| &**boxed)
    }

    /// Get an iterator over all known processes.
    pub fn iter_processes<'this>(
        &'this self,
    ) -> impl Iterator<Item = &'this dyn Process<Fault = Fault>> + 'this {
        self.processes.iter().map(|(_key, boxed)| &**boxed)
    }

    /// Add a process to the list of processes this [`Runtime`] will look after.
    pub fn add_process<P>(&mut self, process: P) -> ProcessIndex
    where
        P: Process<Fault = Fault> + 'static,
    {
        self.processes.insert(Box::new(process))
    }

    /// Poll all known [`Process`]es, removing any that have run to completion
    /// or faulted.
    pub fn poll(&mut self) {
        // set up the device context
        let ctx = Context {
            _devices: &self.devices,
        };
        // we'll need to remember which processes are finished
        let mut to_remove = Vec::new();

        // poll all registered process
        for (pid, process) in &mut self.processes {
            match process.poll(&ctx) {
                Transition::Completed => to_remove.push(pid),
                Transition::StillRunning => {}
                Transition::Fault(_) => unimplemented!(
                    "TODO: Collect all faulted PIDs and their `Fault`s so we can notify the user"
                ),
            }
        }

        // remove all finished processes
        self.processes.retain(|pid, _| !to_remove.contains(&pid));
    }
}

/// Something went wrong...
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Fault {}

/// The interface a [`Process`] can use to interact with the [`Device`]s
/// known by our [`Runtime`].
struct Context<'a> {
    _devices: &'a Devices,
}

impl<'a> rustmatic_core::System for Context<'a> {
    fn get_digital_input(&self, _number: InputNumber) -> Option<bool> {
        unimplemented!(
            "TODO: Figure out which device corresponds to the InputNumber and defer to that"
        )
    }

    fn set_digital_output(&self, _number: OutputNumber, _state: bool) {
        unimplemented!(
            "TODO: Figure out which device corresponds to the OutputNumber and defer to that"
        )
    }

    fn now(&self) -> Instant {
        Instant::now()
    }
}
