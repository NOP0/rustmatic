#![no_std]

extern crate rustmatic_iec_std as iec_std;

use core::time::Duration;
use iec_std::intrinsics::{self, wasm_result_t_WASM_SUCCESS as WASM_SUCCESS};

const INPUT_ADDRESS: usize = 0;
const OUTPUT_ADDRESS: usize = 0;
const TOGGLED_BIT: u8 = 0;

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
            let period = polling_period(INPUT_ADDRESS);
            NEXT_TOGGLE += period;
            iec_std::println!(
                "Next toggle in {:?} at {:?}",
                period,
                NEXT_TOGGLE
            );
            // and make sure we toggle
            CURRENT_STATE = !CURRENT_STATE;
        }

        set_bit_state(OUTPUT_ADDRESS, TOGGLED_BIT, CURRENT_STATE);
    }
}

unsafe fn set_bit_state(address: usize, bit: u8, state: bool) {
    let mut buffer = [0];
    let mask = 1 << bit;

    if state {
        buffer[0] |= mask;
    } else {
        buffer[0] &= !mask;
    }

    let ret = intrinsics::wasm_write_output(
        address as _,
        buffer.as_ptr(),
        buffer.len() as _,
    );
    assert_eq!(ret, WASM_SUCCESS);
}

unsafe fn polling_period(input_address: usize) -> Duration {
    let mut frequency_buf = [0];
    let ret = intrinsics::wasm_read_input(
        input_address as _,
        frequency_buf.as_mut_ptr(),
        frequency_buf.len() as _,
    );
    assert_eq!(ret, WASM_SUCCESS);

    let frequency = frequency_buf[0] as u32;

    if frequency == 0 {
        Duration::from_secs(1)
    } else {
        Duration::from_secs(1) / frequency
    }
}
