#![allow(dead_code)] // used by the panic_handler code when compiling to WASM

use core::{fmt, fmt::Write};

/// A trivial buffer which you can [`write!()`] things to.
///
/// Used to format simple messages when you don't want to pull in `alloc` or
/// 3rd party crates.
pub(crate) struct Buffer<A> {
    buffer: A,
    len: usize,
}

impl<A> Buffer<A> {
    pub const fn new(buffer: A) -> Buffer<A> { Buffer { buffer, len: 0 } }

    pub const fn len(&self) -> usize { self.len }

    pub const fn is_empty(&self) -> bool { self.len() == 0 }
}

impl<A: Array> Buffer<A> {
    pub fn as_str(&self) -> &str {
        let buffer = self.buffer.as_slice();
        core::str::from_utf8(&buffer[..self.len]).unwrap()
    }

    pub fn is_full(&self) -> bool { self.len() == A::CAPACITY }
}

impl<A: Array> Default for Buffer<A> {
    fn default() -> Buffer<A> {
        Buffer {
            buffer: A::zeroed(),
            len: 0,
        }
    }
}

impl<A: Array> Write for Buffer<A> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let end_ix = self.len + s.len();

        let buffer = self.buffer.as_slice_mut();

        if end_ix <= buffer.len() {
            buffer[self.len..end_ix].copy_from_slice(s.as_bytes());
            self.len = end_ix;
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}

/// A hack to work around the lack of const-generics and so we don't need to
/// pull in 3rd party crates.
pub(crate) trait Array {
    const CAPACITY: usize;

    fn zeroed() -> Self;
    fn as_slice(&self) -> &[u8];
    fn as_slice_mut(&mut self) -> &mut [u8];
}

macro_rules! impl_array {
    ($n:expr) => {
        impl Array for [u8; $n] {
            const CAPACITY: usize = $n;

            fn as_slice(&self) -> &[u8] { &self[..] }

            fn as_slice_mut(&mut self) -> &mut [u8] { &mut self[..] }

            fn zeroed() -> Self {
                [0; $n]
            }
        }
    };
    ($first:expr $(, $rest:expr )*) => {
        impl_array!($first);

        impl_array!( $($rest),* );
    };
}

impl_array!(2, 4, 8, 16, 32, 64, 128, 256, 512, 1024);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_bytes_to_string() {
        let mut buffer = Buffer::new([0_u8; 128]);
        let text = "Hello, World!";

        write!(buffer, "{}", text).unwrap();

        assert_eq!(buffer.as_str(), text);
    }

    #[test]
    fn writing_to_a_full_buffer_will_error() {
        let mut buffer = Buffer::new([0; 4]);
        write!(buffer, "asdf").unwrap();
        assert!(buffer.is_full());

        let got = write!(buffer, "g");

        assert!(got.is_err());
    }
}
