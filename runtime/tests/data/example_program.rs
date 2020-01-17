#![no_std]
use rustmatic_iec_std::intrinsics::{
    self, wasm_log_level_LOG_INFO as LOG_INFO,
    wasm_result_t_WASM_SUCCESS as WASM_SUCCESS,
};
use rustmatic_core::System;
use rustmatic_wasm::Environment;

#[no_mangle]
pub extern "C" fn poll(env: &mut dyn Environment) {
    unsafe {
        let file = file!();
        let msg = "Polling\n";

        let ret = intrinsics::wasm_log(
            LOG_INFO,
            file.as_ptr() as *const _,
            file.len() as _,
            line!() as _,
            msg.as_ptr() as *const _,
            msg.len() as _,
        );

        assert_eq!(ret, WASM_SUCCESS);
    }
}
