use rustmatic_core::{Device, DeviceError};
use std::{
    fmt::{self, Display, Formatter},
    time::Instant,
};

#[derive(Clone)]
pub struct DummyBool {
    value: bool,
    created: Instant,
}

impl DummyBool {
    pub fn new() -> Self {
        DummyBool {
            value: false,
            created: Instant::now(),
        }
    }
}

impl Device<bool> for DummyBool {
    fn read(&self) -> Result<bool, DeviceError> {
        let elapsed = self.created.elapsed().as_secs();
        if elapsed % 2 != 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn write(&self, _new_state: bool) -> Result<(), DeviceError> {
        unimplemented! {}
    }
}

impl Display for DummyBool {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "DummyBool value:s {}", self.value)
    }
}

pub struct DummyU32 {
    value: u32,
    created: Instant,
}

impl DummyU32 {
    pub fn new() -> Self {
        DummyU32 {
            value: 0,
            created: Instant::now(),
        }
    }
}

impl Device<u32> for DummyU32 {
    fn read(&self) -> Result<u32, DeviceError> {
        let elapsed = self.created.elapsed().as_secs();

        if elapsed % 2 != 0 {
            Ok(1)
        } else {
            Ok(0)
        }
    }

    fn write(&self, __new_state: u32) -> Result<(), DeviceError> {
        unimplemented! {}
    }
}

impl Display for DummyU32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "DummyU32 value:s {}", self.value)
    }
}
