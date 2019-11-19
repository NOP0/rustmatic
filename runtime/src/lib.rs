//! The system in charge of working with IO and executing processes.

use rustmatic_core::{
    Clock, InputNumber, InputOutput, InputOutputError, OutputNumber, Process,
    System, Transition, Value, VariableError, VariableIndex,
};
use slotmap::DenseSlotMap;
use std::{cell::RefCell, time::Instant};

slotmap::new_key_type! {
    pub struct DeviceIndex;
    pub struct ProcessIndex;
}

type Devices = DenseSlotMap<DeviceIndex, Box<dyn InputOutput<bool>>>;
type Processes = DenseSlotMap<ProcessIndex, Box<dyn Process<Fault = Fault>>>;
type Variables = DenseSlotMap<VariableIndex, Variable>;

/// The PLC runtime.
pub struct Runtime {
    pub(crate) devices: Devices,
    pub(crate) processes: Processes,
    pub(crate) variables: Variables,
}

impl Runtime {
    /// Create an empty [`Runtime`].
    pub fn new() -> Self {
        Runtime {
            devices: Devices::with_key(),
            processes: Processes::with_key(),
            variables: Variables::with_key(),
        }
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
    pub fn poll(&mut self) -> Vec<(ProcessIndex, Fault)> {
        // we'll need to remember which processes are finished
        let mut to_remove = Vec::new();
        let mut faults = Vec::new();

        // poll all registered process
        for (pid, process) in &mut self.processes {
            // set up the device context
            let ctx = Context {
                _devices: &self.devices,
                current_process: pid,
                variables: RefCell::new(&mut self.variables),
            };

            match process.poll(&ctx) {
                Transition::Completed => to_remove.push(pid),
                Transition::StillRunning => {},
                Transition::Fault(fault) => {
                    faults.push((pid, fault));
                },
            }
        }

        // remove all finished processes
        self.processes.retain(|pid, _| !to_remove.contains(&pid));

        faults
    }
}

/// Something went wrong...
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Fault {}

/// The interface a [`Process`] can use to interact with the [`Device`]s
/// known by our [`Runtime`].
struct Context<'a> {
    _devices: &'a Devices,
    variables: RefCell<&'a mut Variables>,
    current_process: ProcessIndex,
}

impl<'a> System for Context<'a> {
    fn digital_io(&self) -> &dyn InputOutput<bool> { self }

    fn variables(&self) -> &dyn rustmatic_core::Variables { self }

    fn clock(&self) -> &dyn Clock { self }
}

impl<'a> rustmatic_core::Variables for Context<'a> {
    fn declare(
        &self,
        name: &str,
        initial_value: Value,
    ) -> Result<VariableIndex, VariableError> {
        let variable = Variable {
            name: String::from(name),
            owner: self.current_process,
            value: RefCell::new(initial_value),
        };

        let existing_declaration = self
            .variables
            .borrow()
            .iter()
            .find(|(_, var)| var.name == name)
            .map(|(key, _var)| key);

        match existing_declaration {
            Some(previous_definition) => {
                Err(VariableError::DuplicateVariable {
                    previous_definition,
                })
            },
            None => Ok(self.variables.borrow_mut().insert(variable)),
        }
    }

    fn read(&self, index: VariableIndex) -> Option<Value> {
        self.variables
            .borrow_mut()
            .get(index)
            .map(|var| var.value.borrow().clone())
    }

    fn set(&self, index: VariableIndex, new_value: Value) {
        if let Some(var) = self.variables.borrow_mut().get(index) {
            let mut value = var.value.borrow_mut();
            *value = new_value;
        }
    }
}

impl<'a> Clock for Context<'a> {
    fn now(&self) -> Instant { Instant::now() }
}

impl<'a> InputOutput<bool> for Context<'a> {
    fn read(&self, _number: InputNumber) -> Result<bool, InputOutputError> {
        unimplemented!()
    }

    fn write(
        &self,
        _number: OutputNumber,
        _value: bool,
    ) -> Result<(), InputOutputError> {
        unimplemented!()
    }
}

/// A [`Variable`] is some value that can be accessed by different parts of the
/// runtime (e.g. another process or a debugger).
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    /// A human-friendly name for the variable.
    pub name: String,
    /// The [`Process`] this variable is associated with.
    pub owner: ProcessIndex,
    /// The variable's current value.
    ///
    /// # Note
    ///
    /// Interior mutability is used because many [`Process`]es may want to
    /// access the [`Value`] or parent [`System`] concurrently.
    pub value: RefCell<Value>,
}
