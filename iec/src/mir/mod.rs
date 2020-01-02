//! Language-agnostic mid-level internal representation.
//!
//! For more details, check out [The IEC 61131-3 Software Model][model].
//!
//! [model]: https://www.automation.com/library/articles-white-papers/coder146s-corner-the-iec-61131-3-software-model

use codespan::{FileId, Span};
use specs::prelude::*;
use specs_derive::Component;
use std::collections::BTreeMap;

pub fn register(world: &mut World) {
    world.register::<Name>();
    world.register::<Location>();
    world.register::<Scope>();
    world.register::<ScopeRef>();
    world.register::<HasType>();
    world.register::<Type>();
    world.register::<Function>();
}

#[derive(Debug, Clone, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Function {
    pub local_variables: Vec<Entity>,
    pub return_types: Vec<Entity>,
    pub parameters: Vec<Entity>,
}

/// The name for a component.
#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
pub struct Name(pub String);

impl<T> PartialEq<T> for Name
where
    str: PartialEq<T>,
    T: ?Sized,
{
    fn eq(&self, other: &T) -> bool { self.0.as_str().eq(other) }
}

/// Where an item is defined within source code.
#[derive(Debug, Clone, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Location {
    pub span: Span,
    pub file: FileId,
}

#[derive(Debug, Clone, PartialEq, Component)]
#[storage(DenseVecStorage)]
pub struct Scope {
    pub parent: Option<Entity>,
    pub symbol_table: BTreeMap<String, Symbol>,
}

impl Scope {
    /// Create a root scope.
    pub fn root() -> Scope {
        Scope {
            parent: None,
            symbol_table: BTreeMap::new(),
        }
    }

    /// Tries to add a [`Symbol`] to the [`Scope`]'s symbol table, erroring if
    /// a [`Symbol`] with that name already exists.
    pub fn add_symbol(
        &mut self,
        name: String,
        value: Symbol,
    ) -> Result<(), DuplicateSymbolError> {
        use std::collections::btree_map::Entry;

        match self.symbol_table.entry(name.clone()) {
            Entry::Vacant(vacancy) => {
                vacancy.insert(value);
                Ok(())
            },
            Entry::Occupied(occupied) => Err(DuplicateSymbolError {
                name,
                original: occupied.get().clone(),
                duplicate: value,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("Tried to declare a {} called \"{}\" but there is already a {} called that", duplicate.description(), name, original.description())]
pub struct DuplicateSymbolError {
    pub name: String,
    pub original: Symbol,
    pub duplicate: Symbol,
}

/// A reference to the [`Scope`] this [`Entity`] is attached to.
#[derive(Debug, Clone, PartialEq, Component)]
#[storage(DenseVecStorage)]
pub struct ScopeRef(pub Entity);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Symbol {
    /// A [`VariableDeclaration`] defined in this scope.
    LocalVariable(Entity),
    /// A global [`Function`].
    Function(Entity),
}
impl Symbol {
    pub fn entity(self) -> Entity {
        match self {
            Symbol::LocalVariable(e) | Symbol::Function(e) => e,
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Symbol::LocalVariable(_) => "variable",
            Symbol::Function(_) => "function",
        }
    }
}

/// Something which has a [`Type`] as resolved by the type system.
#[derive(Debug, Clone, PartialEq, Component)]
#[storage(DenseVecStorage)]
pub struct HasType {
    pub ty: Entity,
}

#[derive(Debug, Clone, PartialEq, Component)]
#[storage(HashMapStorage)]
pub enum Type {
    Integer { signed: bool, bit_width: usize },
}
