#![cfg(all(not(test), target_arch = "wasm32"))]

use core::{fmt, fmt::Write, panic::PanicInfo};

use crate::intrinsics::{self, wasm_log_level_LOG_ERROR as LOG_ERROR};

#[panic_handler]
pub fn panic_handler(info: &PanicInfo) -> ! {
    let mut buffer = Buffer::default();

    unsafe {
        let _ = write!(buffer, "{}", info);
        let msg = buffer.as_str();

        let file = info.location().map(|l| l.file()).unwrap_or("<unknown>");
        let line = info.location().map(|l| l.line()).unwrap_or(0);

        let _ = intrinsics::wasm_log(
            LOG_ERROR,
            file.as_ptr() as *const _,
            file.len() as _,
            line as _,
            msg.as_ptr() as *const _,
            msg.len() as _,
        );

        core::arch::wasm32::unreachable()
    }
}

struct Buffer {
    buffer: [u8; 512],
    cursor: usize,
}

impl Buffer {
    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buffer[..self.cursor]).unwrap()
    }
}

impl Default for Buffer {
    fn default() -> Buffer {
        Buffer {
            buffer: [0; 512],
            cursor: 0,
        }
    }
}

impl Write for Buffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let end_ix = self.cursor + s.len();

        if end_ix < self.buffer.len() {
            self.buffer[self.cursor..end_ix].copy_from_slice(s.as_bytes());
            self.cursor = end_ix;
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}
