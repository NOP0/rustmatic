use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// An individual IO device (e.g. Ethercat bus).
///
/// # Note To Implementors
///
/// Reading or writing should never block.
///
/// The [`Device`] should print a concise human-readable description using
/// [`Display`].
pub trait Device<T>: Display {
    fn read(&self) -> Result<T, DeviceError>;

    fn write(&self, new_state: T) -> Result<(), DeviceError>;
}

#[derive(Debug)]
pub enum DeviceError {
    UnknownNumber,
    Other(Box<dyn Error>),
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DeviceError::UnknownNumber => write!(f, "Unknown Number"),
            DeviceError::Other(ref other) => Display::fmt(other, f),
        }
    }
}

impl Error for DeviceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DeviceError::UnknownNumber => None,
            DeviceError::Other(ref other) => Some(&**other),
        }
    }
}

// We can't implement `From<E> where E: Error` because `DeviceError` implements
// `Error` and we'd overlap with `impl<T> From<T> for T` in std. That makes `?`
// a lot less useful 😞

impl From<Box<dyn Error>> for DeviceError {
    fn from(other: Box<dyn Error>) -> DeviceError { DeviceError::Other(other) }
}
