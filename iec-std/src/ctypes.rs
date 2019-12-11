//! Re-exports of C types on a "normal" x86 computer. Normally you'd use
//! `std::os::raw` or `libc`, but in our case that's not possible.

#![allow(bad_style, dead_code)]

// /src/libc/unix/linux_like/linux/gnu/b64/x86_64/mod.rs
pub type c_char = i8;
pub type wchar_t = i32;

// /src/libc/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs
pub type c_long = i64;
pub type c_ulong = u64;

// src/libc/unix/mod.rs
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;
