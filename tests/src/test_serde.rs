use std::collections::LinkedList;

use serde::{Deserialize, Serialize};
use serde_molecule::{dynvec_serde, from_slice, to_vec};

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
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
    pub f9: LinkedList<[u8; 3]>,
}

#[test]
fn test_serde_1() {
    let mut value = Struct1::default();
    value.f1 = 100;
    value.f2 = 200;
    value.f3 = [1, 2, 3];
    value.f4 = [[1, 2, 3, 4, 5], [1, 2, 3, 4, 5]];
    value.f5 = vec![1, 2, 3];
    value.f6 = String::from("hello");
    value.f8 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![]];
    value.f9 = [[1, 2, 3], [4, 5, 6]].into();
    let bytes = to_vec(&value, false).unwrap();
    let value2 = from_slice(&bytes).unwrap();
    assert_eq!(value, value2);
}
