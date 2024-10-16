use alloc::string::String;
use alloc::string::ToString;
use core::fmt::{self, Debug, Display};
use core::result;
use serde::{de, ser};
#[derive(Debug)]
pub enum Error {
    /// Contains a general error message as a string.
    Message(String),

    /// Occurs when the data length is incorrect while parsing a number or molecule header.
    MismatchedLength,

    /// Occurs when the data length is insufficient while parsing a number or molecule header.
    LengthNotEnough,

    /// Indicates that the method or type is not implemented. Not all types in Rust can be serialized.
    Unimplemented,

    /// Occurs when assembling a molecule fixvec, and the size of each element is inconsistent.
    AssembleFixvec,

    /// Occurs when the header or size is incorrect while parsing a molecule fixvec.
    InvalidFixvec,

    /// Occurs when the field count is mismatched while parsing a molecule table.
    MismatchedTableFieldCount,

    /// Occurs when an overflow happens while parsing a molecule header.
    Overflow,

    /// Indicates an error encountered while parsing a molecule array.
    InvalidArray,

    /// Indicates that non-fixed size fields are not allowed in a molecule struct, e.g., `Option`, `Vec`, `DynVec`, `enum`.
    InvalidStructField,

    /// Indicates that a map should have exactly two fields: a key and a value.
    InvalidMap,

    /// Indicates that the table header is invalid or malformed.
    InvalidTable,

    /// Indicates that the table length is invalid or malformed.
    InvalidTableLength,

    /// Indicates that the table header is invalid or malformed.
    InvalidTableHeader,

    /// Indicates that the field count in serialization is mismatched.
    InvalidTableCount,

    /// Indicates that non-fixed size fields are not allowed in a molecule struct, e.g., `Option`, `Vec`, `DynVec`, `enum`.
    MixTableAndStruct,

    /// Invalid char
    InvalidChar,
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
    fn source(&self) -> Option<&(dyn de::StdError + 'static)> {
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
