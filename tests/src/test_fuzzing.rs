// test cases about bugs found in fuzzing tests

use serde::{Deserialize, Serialize};
use serde_molecule::{dynvec_serde, error::Result, struct_serde};
use serde_molecule::{from_slice, to_vec};

#[derive(Serialize, Deserialize)]
struct Struct0 {
    f0: u8,
    f1: u64,
}

#[derive(Serialize, Deserialize)]
struct Struct1 {
    pub f1: u8,
    pub f2: u16,
    pub f3: [u8; 3],
    pub f4: [[u8; 5]; 2],
    pub f5: Vec<u8>,
    pub f6: String,
    pub f7: Option<u32>,
    #[serde(with = "dynvec_serde")]
    pub f8: Vec<Vec<u8>>,
    #[serde(with = "struct_serde")]
    pub f10: Struct0,
}

fn test_oom() {
    let bytes = include_bytes!("../test-data/oom-dump");
    let _: Result<Struct1> = from_slice(bytes, false);
}
