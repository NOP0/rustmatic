use gpio_cdev::{Chip, LineRequestFlags};
use rustmatic_core::{Device, DeviceError};
use std::{
    cell::RefCell,
    fmt::{self, Display, Formatter},
};

#[derive(PartialEq)]
pub enum Direction {
    Input,
    Output,
}
/// A [`Device`] which reads or writes to a single GPIO pin using Linux's
/// GPIO sysfs interface.
pub struct GpioPin {
    chip: RefCell<Chip>,
    line: u32,
    direction: Direction,
}

impl GpioPin {
    pub const fn input(chip: Chip, line: u32) -> GpioPin {
        GpioPin {
            chip: RefCell::new(chip),
            line,
            direction: Direction::Input,
        }
    }

    pub const fn output(chip: Chip, line: u32) -> GpioPin {
        GpioPin {
            chip: RefCell::new(chip),
            line,
            direction: Direction::Output,
        }
    }
}

impl Device<bool> for GpioPin {
    fn read(&self) -> Result<bool, DeviceError> {
        if self.direction != Direction::Input {
            return Err(DeviceError::Other(
                format!("This is not an input").into(),
            ));
        }

        let handle = self
            .chip
            .borrow_mut()
            .get_line(self.line)
            .map_err(|e| DeviceError::Other(Box::new(e)))?
            .request(LineRequestFlags::INPUT, 1, "gpio_pin")
            .map_err(|e| DeviceError::Other(Box::new(e)))?;

        match handle.get_value() {
            Ok(1) => Ok(true),
            Ok(0) => Ok(false),
            Ok(other_value) => Err(DeviceError::Other(format!("The linux GPIO subsystem should have returned 0 or 1, got {}", other_value).into())),
            Err(e) => Err(DeviceError::Other(Box::new(e))),
            }
    }

    fn write(&self, new_state: bool) -> Result<(), DeviceError> {
        unimplemented!()
        //       let value = if new_state { 1 } else { 0 };

        //       self.inner
        //           .set_value(value)
        //           .map_err(|e| DeviceError::Other(Box::new(e)))
    }
}

impl Display for GpioPin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
        //        write!(f, "Linux GPIO pin {}", self.inner.get_pin_num())
    }
}
