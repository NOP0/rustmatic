//! A parser for the [*Structured Text*][st] programming language.
//!
//! # Optional Features
//!
//! Extra functionality is accessible by enabling feature flags. The features
//! currently available are:
//!
//! - **serde-1** - Serialization using the [`serde`] crate
//!
//! [st]: https://en.wikipedia.org/wiki/Structured_text

#[macro_use]
extern crate pest;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod ast;
mod error;
pub mod parser;

pub use crate::{ast::*, error::ParseError};

use crate::parser::{RawParser, Rule};
use pest::Parser as _;

/// Parse a string of *Structured Text*.
pub fn parse(src: &str) -> Result<File, ParseError> {
    let _p = RawParser::parse(Rule::program, src)?;

    unimplemented!()
}
