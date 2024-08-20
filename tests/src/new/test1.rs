use serde::ser;
use serde::Deserialize;
use serde::Serialize;
use serde_molecule::dynvec_serde;
use serde_molecule::struct_serde;
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

// molecule schemas:
// union EnumCustomizedId {
//     Struct1: 4278190081,
//     Table1:  4278190082,
// }
//
pub enum EnumCustomizedId {
    S1(Struct1),
    T1(Table1),
}

impl Serialize for EnumCustomizedId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            EnumCustomizedId::S1(s1) => {
                let mut data: Vec<u8> = (4278190081u32).to_le_bytes().into();
                data.extend(
                    to_vec(s1, true)
                        .map_err(|_| ser::Error::custom("failed to serialized Struct1"))?,
                );
                serializer.serialize_bytes(&data)
            }
            EnumCustomizedId::T1(t1) => {
                let mut data: Vec<u8> = (4278190082u32).to_le_bytes().into();
                data.extend(
                    to_vec(t1, false)
                        .map_err(|_| ser::Error::custom("failed to serialize Table1"))?,
                );
                serializer.serialize_bytes(&data)
            }
        }
    }
}
