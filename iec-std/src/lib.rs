//! The standard library linked into all IEC programs.

// we are the standard library.
#![no_std]

// public export is used by macro code
#[doc(hidden)]
pub extern crate arrayvec;

pub(crate) mod ctypes;
#[allow(bad_style)]
pub mod intrinsics;
mod macros;
mod sys;
pub mod time;
