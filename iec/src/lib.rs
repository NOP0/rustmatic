//! A compiler for the IEC 61131-3 family of programming languages.

pub mod lowering;
pub mod mir;
mod utils;

pub use utils::Diagnostics;
