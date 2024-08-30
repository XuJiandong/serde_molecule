use core::fmt;

use crate::error::Error;
use crate::ser::to_vec;
use alloc::vec::Vec;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::{
    de::{self, value::U64Deserializer, DeserializeSeed, MapAccess, SeqAccess, Visitor},
    ser, Deserializer, Serialize, Serializer,
};

pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    let data = to_vec(value, true).map_err(ser::Error::custom)?;
    serializer.serialize_bytes(&data)
}

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    // This is a tricky approach: use this method to indicate that it is the top
    // level of the Molecule struct. When the inner `deserialize` is invoked, it
    // does not create a new instance. This ensures that the `index` status
    // propagates across the calling functions. When it is MoleculeDeserializer,
    // it defaults to true, which is exactly what we want.
    if deserializer.is_human_readable() {
        let data = CollectData::deserialize(deserializer)?.data;
        let mut de = MoleculeStructDeserializer::new(data);
        T::deserialize(&mut de).map_err(de::Error::custom)
    } else {
        T::deserialize(deserializer)
    }
}

pub struct CollectData {
    pub data: Vec<u8>,
}

impl<'de> Deserialize<'de> for CollectData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct _Visitor;
        impl<'de> Visitor<'de> for _Visitor {
            type Value = CollectData;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a CollectData")
            }
            fn visit_bytes<E>(self, slice: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CollectData {
                    data: slice.to_vec(),
                })
            }
        }
        let visitor = _Visitor;
        deserializer.deserialize_bytes(visitor)
    }
}

/// A structure that deserializes molecule struct into Rust values.
pub(crate) struct MoleculeStructDeserializer {
    data: Vec<u8>,
    index: usize,
}

impl MoleculeStructDeserializer {
    pub fn new(data: Vec<u8>) -> Self {
        MoleculeStructDeserializer { data, index: 0 }
    }
}

macro_rules! read_primitive {
    ($method:ident, $type: ty, $len: expr) => {
        fn $method(&mut self) -> Result<$type, Error> {
            if self.data.len() < (self.index + $len) {
                return Err(Error::MismatchedLength);
            } else {
                let bytes: [u8; $len] = (&self.data[self.index..self.index + $len])
                    .try_into()
                    .unwrap();
                self.index += $len;
                Ok(<$type>::from_le_bytes(bytes))
            }
        }
    };
}

impl MoleculeStructDeserializer {
    read_primitive!(read_u8, u8, 1);
    read_primitive!(read_u16, u16, 2);
    read_primitive!(read_u32, u32, 4);
    read_primitive!(read_u64, u64, 8);
    read_primitive!(read_u128, u128, 16);
    read_primitive!(read_i8, i8, 1);
    read_primitive!(read_i16, i16, 2);
    read_primitive!(read_i32, i32, 4);
    read_primitive!(read_i64, i64, 8);
    read_primitive!(read_i128, i128, 16);
    read_primitive!(read_f32, f32, 4);
    read_primitive!(read_f64, f64, 8);
}

impl<'de, 'a> Deserializer<'de> for &'a mut MoleculeStructDeserializer {
    type Error = Error;

    fn is_human_readable(&self) -> bool {
        false
    }
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.read_u8()? == 0 {
            visitor.visit_bool(false)
        } else {
            visitor.visit_bool(true)
        }
    }
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.read_i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.read_i16()?)
    }
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.read_i32()?)
    }
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.read_i64()?)
    }
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i128(self.read_i128()?)
    }
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.read_u8()?)
    }
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.read_u16()?)
    }
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.read_u32()?)
    }
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.read_u64()?)
    }
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u128(self.read_u128()?)
    }
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.read_f32()?)
    }
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.read_f64()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.read_u32()?)
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::InvalidStructField)
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::InvalidStructField)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bytes(&self.data[self.index..])
    }
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::InvalidStructField)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::InvalidStructField)
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::InvalidStructField)
    }

    /// Parses a newtype struct as the underlying value.
    fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::InvalidStructField)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let access = ArrayAccess::new(self, len);
        visitor.visit_seq(access)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::InvalidStructField)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(StructAccess::new(self, fields.len()))
    }
    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::InvalidStructField)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }
}

struct StructAccess<'a> {
    de: &'a mut MoleculeStructDeserializer,
    current_index: usize,
    count: usize,
}

impl<'a> StructAccess<'a> {
    fn new(de: &'a mut MoleculeStructDeserializer, count: usize) -> Self {
        StructAccess {
            de,
            current_index: 0,
            count,
        }
    }
}

impl<'de, 'a> MapAccess<'de> for StructAccess<'a> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.current_index < self.count {
            let de = U64Deserializer::<Error>::new(self.current_index as u64);
            Ok(Some(seed.deserialize(de)?))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        assert!(self.current_index < self.count);
        self.current_index += 1;
        seed.deserialize(&mut *self.de)
    }
}

struct ArrayAccess<'a> {
    de: &'a mut MoleculeStructDeserializer,
    current_index: usize,
    count: usize,
}

impl<'a> ArrayAccess<'a> {
    pub fn new(de: &'a mut MoleculeStructDeserializer, count: usize) -> Self {
        ArrayAccess {
            de,
            current_index: 0,
            count,
        }
    }
}

impl<'de, 'a> SeqAccess<'de> for ArrayAccess<'a> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.current_index < self.count {
            self.current_index += 1;
            let value = seed.deserialize(&mut *self.de)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}
