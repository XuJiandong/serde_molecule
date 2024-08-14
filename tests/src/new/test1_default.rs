use std::collections::BTreeMap;

use super::test1::Struct1;
use lazy_static::lazy_static;

pub const DEFAULT_BYTE: u8 = 0xcd;
pub const DEFAULT_U16: u16 = 0xcd;
pub const DEFAULT_U32: u32 = 0xcd;
pub const DEFAULT_U64: u64 = 0xcd;
pub const DEFAULT_U128: u128 = 0xcd;

lazy_static! {
    pub static ref DEFAULT_FIXVEC: Vec<u8> = vec![0xcd, 0xcd, 0xcd];
    pub static ref DEFAULT_DYNVEC: Vec<Vec<u8>> = {
        vec![
            vec![0xcd, 0xcd, 0xcd],
            vec![0xcd, 0xcd, 0xcd],
            vec![0xcd, 0xcd, 0xcd],
        ]
    };
    pub static ref DEFAULT_STRUCT1: Struct1 = Struct1 { f1: 0xcd, f2: 0xcd };
    pub static ref DEFAULT_ARRAY3: [u8; 3] = [0xcd, 0xcd, 0xcd];
    pub static ref DEFAULT_STRING: String = String::from("cd");
    pub static ref DEFAULT_MAP: BTreeMap<u128, Vec<u8>> = {
        let mut map = BTreeMap::new();
        map.insert(DEFAULT_U128, DEFAULT_FIXVEC.clone());
        map
    };
}
