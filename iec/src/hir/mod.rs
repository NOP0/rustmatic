//! The high-level intermediate representation of an IEC 61131-3 application.

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
