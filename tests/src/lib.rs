#![doc(hidden)]
#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(dead_code)]

use serde::{de::DeserializeOwned, Serialize};
use serde_molecule::{from_slice, to_vec};
use std::fmt::Debug;
pub mod ckb_types;
pub mod new;
pub mod old;
pub mod simple;
pub mod test_big_array;
pub mod test_ckb_types;
pub mod test_fuzzing;
pub mod test_serde;

pub fn test_once<V: Serialize + DeserializeOwned>(value: &V) {
    let bytes = to_vec(value, false).expect("Failed to serialize value");
    let value2: V = from_slice(&bytes, false).expect("Failed to deserialize value");
    let bytes2 = to_vec(&value2, false).expect("Failed to re-serialize value");
    assert_eq!(bytes, bytes2, "Re-serialized bytes do not match original");
}

pub fn test_eq_once<V>(value: &V)
where
    V: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let bytes = to_vec(value, false).expect("Failed to serialize value");
    let value2: V = from_slice(&bytes, false).expect("Failed to deserialize value");
    assert_eq!(value, &value2, "Re-serialized bytes do not match original");
}

pub fn compare_slice(s1: &[u8], s2: &[u8]) {
    if s1.len() != s2.len() {
        println!("length mismatched: {} vs {}", s1.len(), s2.len());
    }
    let min = s1.len().min(s2.len());
    for i in 0..min {
        if s1[i] != s2[i] {
            println!("byte at index {} are mismatched: {} {}", i, s1[i], s2[i]);
            assert!(false);
        }
    }
}
