#![no_std]

use rustmatic_iec_std::intrinsics::{
    self, wasm_result_t_WASM_SUCCESS as WASM_SUCCESS,
};

const ADDRESS: u32 = 1;

#[no_mangle]
pub extern "C" fn poll() {
    let payload = [1, 2, 3, 4, 5];

    unsafe {
        let ret = intrinsics::wasm_write_output(
            ADDRESS,
            payload.as_ptr(),
            payload.len() as _,
        );
        assert_eq!(ret, WASM_SUCCESS);
    }
}
