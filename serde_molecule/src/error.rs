use core::result;
use serde::ser;
#[cfg(feature = "std")]
use std::error;
use std::fmt::{self, Debug, Display};

pub enum Error {
    Unknown,
    Message(String),
    MismatchedLength,
}

pub type Result<T> = result::Result<T, Error>;

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::Message(msg.to_string())
    }
}

impl serde::de::StdError for Error {
    #[cfg(feature = "std")]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error({:?})", self)
    }
}