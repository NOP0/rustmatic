//! The underlying [`pest`] parser module. You probably don't want to use this
//! directly.

/// The raw [`pest::Parser`] type.
#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct Parser;
