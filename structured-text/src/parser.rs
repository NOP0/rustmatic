
/// The raw [`pest::Parser`] type.
#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct Parser;