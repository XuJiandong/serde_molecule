use serde::Deserialize;
use serde::Serialize;
use serde::{de, ser};
use serde_molecule::de::MoleculeDeserializer;
use serde_molecule::dynvec_serde;
use serde_molecule::struct_serde::{self, CollectData};
use serde_molecule::to_vec;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Struct1 {
    pub f1: u8,
    pub f2: u16,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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
    pub option: Option<u128>,
    pub array3: [u8; 3],
    pub string: String,
    #[serde(with = "struct_serde")]
    pub struct1_opt: Option<Struct1>,
    pub map: BTreeMap<u128, Vec<u8>>,
}

#[derive(Serialize)]
pub enum Enum1 {
    U16(u16),
    U32(u32),
}
