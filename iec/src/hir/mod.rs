//! The high-level intermediate representation of an IEC 61131-3 application.

use specs::Entity;

/// the language element which corresponds to a *programmable controller system*
/// as defined in IEC 61131-1.
///
/// Can be started and stopped via the:
///
/// - Operator interface
/// - Programming, testing, and monitoring functions
/// - Operating system functions
///
/// Starting a [`Configuration`] will initialize all global variables then start
/// all [`Resource`]s in the configuration.
///
/// Stopping a [`Configuration`] will stop all of its [`Resource`]s.
#[derive(Debug, Clone, PartialEq)]
pub struct Configuration {
    pub resources: Vec<Resource>,
}

/// A *"signal processing function"* and its *"man-machine interface"* and
/// *"sensor and actuator interface"* functions.
///
/// Starting a [`Resource`] will first initialize all variables inside it, then
/// enable all of the [`Task`]s in the resource.
///
/// Stopping a [`Resource`] will disable all associated [`Task`]s.
#[derive(Debug, Clone, PartialEq)]
pub struct Resource {
    pub programs: Vec<Program>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Task {}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub blocks: Vec<FunctionBlock>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBlock {}

/// A variable.
///
/// Variable values within a program can be communicated directly by connection
/// of the output of one program element to the input of another. Global
/// variables can be used to communicate between programs within the same
/// [`Configuration`].
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub name: String,
    /// The [`Type`] this [`Type`] inherits from.
    pub parent: Option<Entity>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Builtin,
    Array(Array),
    Enum(Enumeration),
    Composite(Struct),
    BoundedInteger(BoundedInteger),
}

/// An array of homogeneous items.
#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    pub length: usize,
    /// The [`Type`] of the items in this array.
    pub element_type: Entity,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enumeration {
    /// The [`EnumerationVariant`]s associated with this [`Enumeration`].
    pub variants: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumerationVariant {
    pub name: String,
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {}

#[derive(Debug, Clone, PartialEq)]
pub struct BoundedInteger {
    /// The underlying integer [`Type`].
    pub underlying_type: Entity,
    pub minimum_value: i64,
    pub maximum_value: i64,
}
