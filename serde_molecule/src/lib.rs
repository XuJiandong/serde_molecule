#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../../README.md")]

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! {
    "serde_molecule requires that either `std` (default) or `alloc` feature is enabled"
}

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use crate::de::from_slice;
pub use crate::error::{Error, Result};
pub use crate::ser::to_vec;

pub mod big_array_serde;
pub mod de;
pub mod dynvec_serde;
pub mod error;
pub mod molecule;
pub mod ser;
pub mod struct_serde;
#[cfg(test)]
mod tests;
