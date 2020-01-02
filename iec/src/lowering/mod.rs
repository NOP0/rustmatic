//! Lowering from individual language ASTs (e.g. *Structured Text*) to our
//! mid-level internal representation.

mod st;

pub use st::lower as structured_text;

use specs::Entity;

#[derive(Debug, Clone, PartialEq)]
pub struct CompilationUnit {
    /// The root [`crate::mir::Scope`].
    pub global_scope: Entity,
}
