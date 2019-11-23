use rustmatic_core::{
    Device, DeviceError, DeviceRegistrar, InputNumber, OutputNumber,
};
use std::fmt::{self, Display, Formatter};
use sysfs_gpio::Pin;

/// A [`Device`] which reads or writes to a single GPIO pin using Linux's
/// GPIO sysfs interface.
#[derive(Debug, Clone, PartialEq)]
pub struct GpioPin {
    inner: Pin,
    input_number: Option<InputNumber>,
    output_number: Option<OutputNumber>,
}

impl GpioPin {
    pub const fn input(number: InputNumber, pin: Pin) -> GpioPin {
        GpioPin {
            inner: pin,
            input_number: Some(number),
            output_number: None,
        }
    }

    pub const fn output(number: OutputNumber, pin: Pin) -> GpioPin {
        GpioPin {
            inner: pin,
            input_number: None,
            output_number: Some(number),
        }
    }

    pub const fn bidirectional(
        input_number: InputNumber,
        output_number: OutputNumber,
        pin: Pin,
    ) -> GpioPin {
        GpioPin {
            inner: pin,
            input_number: Some(input_number),
            output_number: Some(output_number),
        }
    }

    pub const fn pin(&self) -> Pin { self.inner }

    pub const fn input_number(&self) -> Option<InputNumber> {
        self.input_number
    }

    pub const fn output_number(&self) -> Option<OutputNumber> {
        self.output_number
    }
}

impl Device<bool> for GpioPin {
    fn register(&self, registrar: &mut dyn DeviceRegistrar) {
        if let Some(input_number) = self.input_number {
            registrar.input(input_number);
        }
        if let Some(output_number) = self.output_number {
            registrar.output(output_number);
        }
    }

    fn read(&self, number: InputNumber) -> Result<bool, DeviceError> {
        if self.input_number != Some(number) {
            return Err(DeviceError::UnknownNumber);
        }

        match self.inner.get_value() {
                Ok(0) => Ok(false),
                Ok(1) => Ok(true),
                Ok(other_value) => Err(DeviceError::Other(format!("The linux GPIO subsystem should have returned 0 or 1, got {}", other_value).into())),
                Err(e) => Err(DeviceError::Other(Box::new(e))),
            }
    }

    fn write(
        &self,
        number: OutputNumber,
        new_state: bool,
    ) -> Result<(), DeviceError> {
        if self.output_number != Some(number) {
            return Err(DeviceError::UnknownNumber);
        }

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
