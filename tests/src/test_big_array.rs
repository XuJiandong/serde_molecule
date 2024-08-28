use serde::{Deserialize, Serialize};
use serde_molecule::big_array_serde;

use crate::test_once;

#[derive(Serialize, Deserialize)]
struct BigArray {
    f1: u8,
    #[serde(with = "big_array_serde")]
    f2: [u8; 33],
    #[serde(with = "big_array_serde")]
    f3: [u8; 64],
}

#[test]
fn test_big_array() {
    let value = BigArray {
        f1: 100,
        f2: [1u8; 33],
        f3: [2u8; 64],
    };
    test_once(&value);
}

#[derive(Serialize, Deserialize)]
struct BigArray2 {
    f1: u8,
    #[serde(with = "big_array_serde")]
    f2: [[u8; 2]; 33],
    #[serde(with = "big_array_serde")]
    f3: [[u8; 3]; 64],
}

#[test]
fn test_big_array_nested() {
    let value = BigArray2 {
        f1: 100,
        f2: [[1u8, 1u8]; 33],
        f3: [[2u8, 2u8, 2u8]; 64],
    };
    test_once(&value);
}
