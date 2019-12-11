#![cfg(all(not(test), target_arch = "wasm32"))]

use core::{fmt, fmt::Write, panic::PanicInfo};

use crate::{
    buffer::Buffer,
    intrinsics::{self, wasm_log_level_LOG_ERROR as LOG_ERROR},
};

#[panic_handler]
pub fn panic_handler(info: &PanicInfo) -> ! {
    let mut buffer = Buffer::new([0; 512]);

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
