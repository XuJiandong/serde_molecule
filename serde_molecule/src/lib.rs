#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! {
    "serde_molecule requires that either `std` (default) or `alloc` feature is enabled"
}

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use crate::error::{Error, Result};
pub use crate::ser::to_vec;
#[cfg(feature = "std")]
pub use crate::ser::Serializer;

pub mod attribute;
pub mod error;
pub mod ser;
