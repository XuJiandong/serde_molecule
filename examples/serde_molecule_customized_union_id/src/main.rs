use serde::Deserialize;
use serde::Serialize;
use serde::{de, ser};
use serde_molecule::de::MoleculeDeserializer;
use serde_molecule::from_slice;
use serde_molecule::struct_serde::CollectData;
use serde_molecule::to_vec;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Struct1 {
    pub f1: u8,
    pub f2: u16,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Table1 {
    pub f1: u8,
    pub f2: Vec<u8>,
}

// molecule schemas:
// union UnionCustomizedId {
//     Struct1: 4278190081,
//     Table1:  4278190082,
// }
//
#[derive(Clone, PartialEq, Debug)]
pub enum UnionCustomizedId {
    S1(Struct1),
    T1(Table1),
}

// example of serialization of customized union id
impl Serialize for UnionCustomizedId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            UnionCustomizedId::S1(s1) => {
                let mut data: Vec<u8> = (4278190081u32).to_le_bytes().into();
                data.extend(
                    to_vec(s1, false)
                        .map_err(|_| ser::Error::custom("failed to serialized Struct1"))?,
                );
                serializer.serialize_bytes(&data)
            }
            UnionCustomizedId::T1(t1) => {
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

// example of deserialization of customized union id
impl<'de> Deserialize<'de> for UnionCustomizedId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = CollectData::deserialize(deserializer)?.data;
        let id = u32::from_le_bytes(data.as_slice()[0..4].try_into().unwrap());
        let mut de = MoleculeDeserializer::new(&data[4..]);

        match id {
            4278190081u32 => {
                let s1 = Struct1::deserialize(&mut de)
                    .map_err(|e| de::Error::custom(format!("{}", e)))?;
                Ok(UnionCustomizedId::S1(s1))
            }
            4278190082u32 => {
                let t1 = Table1::deserialize(&mut de)
                    .map_err(|e| de::Error::custom(format!("{}", e)))?;
                Ok(UnionCustomizedId::T1(t1))
            }
            _ => Err(de::Error::custom("invalid union id")),
        }
    }
}

fn test_once(value: &UnionCustomizedId) {
    let bytes = to_vec(&value, false).unwrap();
    let value2: UnionCustomizedId = from_slice(&bytes, false).unwrap();
    assert_eq!(value, &value2);
}

fn main() {
    let value = UnionCustomizedId::S1(Struct1 { f1: 100, f2: 200 });
    test_once(&value);
}
