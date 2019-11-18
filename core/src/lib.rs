use std::time::Instant;

/// The interface exposed to a [`Process`] so it can interact with the outside
/// world.
pub trait System {
    fn get_digital_input(&self, number: InputNumber) -> Option<bool>;
    fn set_digital_output(&self, number: OutputNumber, state: bool);
    fn now(&self) -> Instant;
}

/// An opaque handle that can be used to read from an input.
pub struct InputNumber(usize);

/// An opaque handle that can be used to write to an output.
pub struct OutputNumber(usize);

/// A single thread of execution.
pub trait Process {
    type Fault;

    fn run(&mut self, system: &dyn System) -> Transition<Self::Fault>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Transition<F> {
    Completed,
    Fault(F),
    StillRunning,
}
