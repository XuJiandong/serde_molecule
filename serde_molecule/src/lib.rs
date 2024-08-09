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
pub use crate::ser::MoleculeSerializer;

pub mod attribute;
pub mod dynvec_serde;
pub mod error;
pub mod ser;
pub mod struct_serde;
