use rustmatic_core::{
    Device, DeviceError};
use std::fmt::{self, Display, Formatter};
use sysfs_gpio::Pin;

/// A [`Device`] which reads or writes to a single GPIO pin using Linux's
/// GPIO sysfs interface.
#[derive(Debug, Clone, PartialEq)]
pub struct GpioPin {
    inner: Pin,
}

impl GpioPin {
    pub const fn input(pin: Pin) -> GpioPin {
        GpioPin {
            inner: pin,
        }
    }

    pub const fn output(pin: Pin) -> GpioPin {
        GpioPin {
            inner: pin,
        }
    }

    pub const fn pin(&self) -> Pin { self.inner }

}

impl Device<bool> for GpioPin {

    fn read(&self) -> Result<bool, DeviceError> {

        match self.inner.get_value() {
                Ok(0) => Ok(false),
                Ok(1) => Ok(true),
                Ok(other_value) => Err(DeviceError::Other(format!("The linux GPIO subsystem should have returned 0 or 1, got {}", other_value).into())),
                Err(e) => Err(DeviceError::Other(Box::new(e))),
            }
    }

    fn write(
        &self,
        new_state: bool,
    ) -> Result<(), DeviceError> {

        let value = if new_state { 1 } else { 0 };

        self.inner
            .set_value(value)
            .map_err(|e| DeviceError::Other(Box::new(e)))
    }
}

impl Display for GpioPin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Linux GPIO pin {}", self.inner.get_pin_num())
    }
}
