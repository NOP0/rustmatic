#![no_std]

extern crate rustmatic_iec_std as iec_std;

use core::time::Duration;
use iec_std::intrinsics::{self, wasm_result_t_WASM_SUCCESS as WASM_SUCCESS};

const ADDRESS: usize = 0;
const BIT: u8 = 0;

static mut NEXT_TOGGLE: Duration = Duration::from_secs(0);
static mut CURRENT_STATE: bool = false;

#[no_mangle]
pub extern "C" fn poll() {
    unsafe {
        let mut secs = 0;
        let mut nanos = 0;
        let ret = intrinsics::wasm_current_time(&mut secs, &mut nanos);
        assert_eq!(ret, WASM_SUCCESS);

        let now = Duration::new(secs, nanos);
        iec_std::println!("Now: {:?}, Current state: {}", now, CURRENT_STATE);

        if now > NEXT_TOGGLE {
            // figure out we need to toggle again
            NEXT_TOGGLE += polling_period();
            // and make sure we toggle
            CURRENT_STATE = !CURRENT_STATE;
        }

        set_bit_state(ADDRESS, BIT, CURRENT_STATE);
    }
}

unsafe fn set_bit_state(address: usize, bit: u8, state: bool) {
    // read the current value
    let mut buffer = [0];
    let ret = intrinsics::wasm_read_input(
        address as u32,
        buffer.as_mut_ptr(),
        buffer.len() as i32,
    );
    assert_eq!(ret, WASM_SUCCESS);

    let mask = 1 << bit;

    if state {
        buffer[0] |= mask;
    } else {
        buffer[0] &= !mask;
    }

    iec_std::println!("Buffer: {:?}", buffer);

    let ret = intrinsics::wasm_write_output(
        address as u32,
        buffer.as_ptr(),
        buffer.len() as i32,
    );
    assert_eq!(ret, WASM_SUCCESS);
}

unsafe fn polling_period() -> Duration { Duration::from_millis(300) }
