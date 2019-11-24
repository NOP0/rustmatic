//! A parser for the [*Structured Text*][st] programming language.
//!
//! [st]: https://en.wikipedia.org/wiki/Structured_text

mod ast;
mod error;
pub mod parser;

pub use crate::{ast::*, error::ParseError};

use crate::parser::{Parser, Rule};
use pest::Parser as _;

/// Parse a string of *Structured Text*.
pub fn parse(src: &str) -> Result<(), ParseError> {
    let _p = Parser::parse(Rule::program, src)?;

    unimplemented!()
}
