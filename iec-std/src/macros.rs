#[macro_export]
macro_rules! print {
    ($( $fmt:tt ),*) => {
        use core::fmt::Write as _;

        let mut msg: $crate::arrayvec::ArrayString<[u8; 512]> = $crate::arrayvec::ArrayString::new();
        if let Err(e) =  write!(msg, $( $fmt ),*) {
            panic!("Unable to write to the buffer: {}", e);
        }

        unsafe {
            let file = file!();
            let line = line!();
            let ret = $crate::intrinsics::wasm_log($crate::intrinsics::wasm_log_level_LOG_INFO,
                file.as_ptr() as *const _,
                file.len() as _,
                line as _,
                msg.as_ptr() as *const _,
                msg.len() as _,
            );
            assert_eq!(ret, $crate::intrinsics::wasm_result_t_WASM_SUCCESS);
        }
    };
}

#[macro_export]
macro_rules! println {
    ($($fmt:tt),*) => {
        $crate::print!($($fmt),*);
    };
}
