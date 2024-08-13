use serde::Serialize;
use serde_molecule::dynvec_serde;
use serde_molecule::struct_serde;

#[derive(Serialize, Clone)]
pub struct Struct1 {
    pub f1: u8,
    pub f2: u16,
}

#[derive(Serialize)]
pub struct Table1 {
    pub f1: u8,
    pub f2: u16,
    pub f3: u32,
    pub f4: u64,
    pub f5: u128,
    pub fixvec: Vec<u8>,
    #[serde(with = "dynvec_serde")]
    pub dynvec: Vec<Vec<u8>>,
    #[serde(with = "struct_serde")]
    pub struct1: Struct1,
}
