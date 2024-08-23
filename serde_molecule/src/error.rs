#[cfg(not(feature = "std"))]
use crate::alloc::string::ToString;
#[cfg(not(feature = "std"))]
use alloc::string::String;
use core::fmt::{self, Debug, Display};
use core::result;
use serde::{de, ser};
#[cfg(feature = "std")]
use std::error;

#[derive(Debug)]
pub enum Error {
    Unknown,
    Message(String),
    MismatchedLength,
    LengthNotEnough,
    Unimplemented,
    InvalidFixvec,
    InvalidDynvec,
    MismatchedTableFieldCount,
    Overflow,
    InvalidArray,
    InvalidStructField,
    InvalidMap,
    InvalidTable,
    InvalidTableLength,
    InvalidTableHeader,
    InvalidTableCount,
}

pub type Result<T> = result::Result<T, Error>;

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::Message(msg.to_string())
    }
}

impl de::StdError for Error {
    #[cfg(feature = "std")]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Message(m) => f.write_str(m),
            _ => {
                write!(f, "{:?}", self)
            }
        }
    }
}
