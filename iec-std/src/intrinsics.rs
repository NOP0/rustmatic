//! Functions provided by the runtime.

#[link(wasm_import_module = "env")]
extern "C" {
    /// Get a measurement of a monotonically nondecreasing clock.
    ///
    /// The absolute numbers don't necessarily mean anything, the difference
    /// between two measurements can be used to tell how much time has passed.
    pub fn current_time(seconds: *mut u64, nanos: *mut u32) -> i32;
}
