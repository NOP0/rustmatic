//! Language-agnostic mid-level internal representation.
//!
//! For more details, check out [The IEC 61131-3 Software Model][model].
//!
//! [model]: https://www.automation.com/library/articles-white-papers/coder146s-corner-the-iec-61131-3-software-model

use codespan::{FileId, Span};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Clone, PartialEq, Component)]
#[storage(HashMapStorage)]
pub struct Configuration {
    pub resources: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Resource {}

#[derive(Debug, Clone, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Program {}

/// A control element for starting one or more programs.
///
/// IEC 61131 defines a task as:
///
/// > An execution control element which is capable of invoking, either on a
/// > periodic basis or upon the occurrence of the rising edge of a specified
/// > Boolean variable, the execution of a set of program organization units,
/// > which can include programs and function blocks whose instances are
/// > specified in the declaration of programs.
#[derive(Debug, Clone, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Task {
    pub trigger: Trigger,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Trigger {
    Periodic,
    RisingEdge { variable: Entity },
}

/// The name for a component.
#[derive(Debug, Clone, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Name(pub String);

/// Where an item is defined within source code.
#[derive(Debug, Clone, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Location {
    pub span: Span,
    pub file: FileId,
}
