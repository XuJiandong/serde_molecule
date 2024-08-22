#![no_std]

use serde::{Deserialize, Serialize};
use serde_molecule::{from_slice, to_vec};

#[derive(Serialize, Deserialize)]
pub struct Table1 {
    pub f1: u8,
    pub f2: u16,
}

fn main() {
    let t1 = Table1 { f1: 0, f2: 0 };
    // serialize
    let bytes = to_vec(&t1, false).unwrap();
    // deserialize
    let t2: Table1 = from_slice(&bytes, false).unwrap();
    assert_eq!(t1.f1, t2.f1);
}
