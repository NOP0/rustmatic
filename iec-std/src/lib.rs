//! The standard library linked into all IEC programs.

// we are the standard library.
#![no_std]

pub(crate) mod ctypes;
#[allow(bad_style)]
pub mod intrinsics;
pub mod time;
