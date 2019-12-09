//! The system in charge of working with IO and executing processes.

use rustmatic_core::{
    DeviceManager, Process, ProcessImage, System, Transition, Value,
    VariableIndex,
};
use slotmap::DenseSlotMap;
use std::{cell::RefCell, time::Instant};

slotmap::new_key_type! {
    pub struct DeviceIndex;
    pub struct ProcessIndex;
}

type Processes = DenseSlotMap<ProcessIndex, Box<dyn Process<Fault = Fault>>>;
type Variables = DenseSlotMap<VariableIndex, Variable>;

/// The PLC runtime.
pub struct Runtime {
    pub devices: DeviceManager,
    pub inputs: ProcessImage,
    pub outputs: ProcessImage,
    pub(crate) processes: Processes,
    pub(crate) variables: Variables,
}

impl Runtime {
    /// Create an empty [`Runtime`].
    pub fn new() -> Self {
        Runtime {
            devices: DeviceManager::new(),
            inputs: ProcessImage::new(),
            outputs: ProcessImage::new(),
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

        self.inputs.update_inputs(&mut self.devices);

        for (pid, process) in &mut self.processes {
            // set up the device context
            let mut ctx = Context {
                devices: &self.devices,
                inputs: &self.inputs,
                outputs: &self.outputs,
                current_process: pid,
                variables: RefCell::new(&mut self.variables),
            };

            match process.poll(&mut ctx) {
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

    pub fn init(&mut self) -> Result<(), Fault> {
        for (pid, process) in &mut self.processes {
            // set up the device context
            let mut ctx = Context {
                devices: &self.devices,
                inputs: &self.inputs,
                outputs: &self.outputs,
                current_process: pid,
                variables: RefCell::new(&mut self.variables),
            };
            process.init(&mut ctx)?;
        }
        Ok(())
    }
}

/// Something went wrong...
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Fault {}

/// The interface a [`Process`] can use to interact with the [`Device<T>`]s
/// known by our [`Runtime`].
struct Context<'a> {
    devices: &'a DeviceManager,
    inputs: &'a ProcessImage,
    outputs: &'a ProcessImage,
    variables: RefCell<&'a mut Variables>,
    current_process: ProcessIndex,
}

impl<'a> System for Context<'a> {
    fn devices(&self) -> &DeviceManager { self.devices }

    fn now(&self) -> Instant { Instant::now() }

    fn declare_variable(
        &self,
        name: &str,
        initial_value: Value,
    ) -> VariableIndex {
        let variable = Variable {
            name: String::from(name),
            owner: self.current_process,
            value: RefCell::new(initial_value),
        };

        self.variables.borrow_mut().insert(variable)
    }

    fn read_variable(&self, index: VariableIndex) -> Option<Value> {
        self.variables
            .borrow_mut()
            .get(index)
            .map(|var| var.value.borrow().clone())
    }

    fn set_variable(&self, index: VariableIndex, new_value: Value) {
        if let Some(var) = self.variables.borrow_mut().get(index) {
            let mut value = var.value.borrow_mut();
            *value = new_value;
        }
    }

    fn inputs(&self) -> &ProcessImage { self.inputs}

    fn outputs(&self) -> &ProcessImage { self.outputs }
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
