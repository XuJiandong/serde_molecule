//! Deserialize molecule data to a Rust data structure.
// TODO: remove it
#![allow(unused_variables)]
#![allow(dead_code)]
use crate::{
    error::{Error, Result},
    molecule::{disassemble_fixvec, disassemble_table, unpack_number},
};
use serde::de::{self, value::U64Deserializer};

//////////////////////////////////////////////////////////////////////////////

/// A structure that deserializes molecule into Rust values.
pub(crate) struct MoleculeDeserializer<'de> {
    data: &'de [u8],
}

impl<'de> MoleculeDeserializer<'de> {
    pub fn new(data: &'de [u8]) -> Self {
        MoleculeDeserializer { data }
    }
}

macro_rules! as_primitives {
    ($method:ident, $type: ty, $len: expr) => {
        fn $method(&self) -> Result<$type> {
            if self.data.len() != $len {
                return Err(Error::MismatchedLength);
            } else {
                let bytes: [u8; $len] = self.data.try_into().unwrap();
                Ok(<$type>::from_le_bytes(bytes))
            }
        }
    };
}

impl<'de> MoleculeDeserializer<'de> {
    pub fn end(self) -> Result<()> {
        todo!()
    }
    as_primitives!(as_u8, u8, 1);
    as_primitives!(as_u16, u16, 2);
    as_primitives!(as_u32, u32, 4);
    as_primitives!(as_u64, u64, 8);
    as_primitives!(as_u128, u128, 16);
    as_primitives!(as_i8, i8, 1);
    as_primitives!(as_i16, i16, 2);
    as_primitives!(as_i32, i32, 4);
    as_primitives!(as_i64, i64, 8);
    as_primitives!(as_i128, i128, 16);
    as_primitives!(as_f32, f32, 4);
    as_primitives!(as_f64, f64, 8);
    // fixvec with element size = 1
    fn disassemble_bytes(&self) -> Result<&'de [u8]> {
        let item_count = unpack_number(self.data, 0)?;
        if item_count != (self.data.len() - 4) {
            Err(Error::InvalidFixvec)
        } else {
            Ok(&self.data[4..])
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut MoleculeDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.as_u8()? == 0 {
            visitor.visit_bool(false)
        } else {
            visitor.visit_bool(true)
        }
    }
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.as_i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.as_i16()?)
    }
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.as_i32()?)
    }
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.as_i64()?)
    }
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i128(self.as_i128()?)
    }
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.as_u8()?)
    }
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.as_u16()?)
    }
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.as_u32()?)
    }
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.as_u64()?)
    }
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u128(self.as_u128()?)
    }
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(self.as_f32()?)
    }
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(self.as_f64()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let s = self.disassemble_bytes()?;
        let v = String::from_utf8_lossy(s);
        visitor.visit_string(v.into_owned())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bytes(self.data)
    }
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.data.is_empty() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    /// Parses a newtype struct as the underlying value.
    fn deserialize_newtype_struct<V>(self, name: &str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let _ = name;
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut access = FixvecAccess::new(self);
        access.parse()?;
        visitor.visit_seq(access)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut access = ArrayAccess::new(self, len);
        access.parse()?;
        visitor.visit_seq(access)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(TableAccess::new(self, 0))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut access = TableAccess::new(self, fields.len());
        access.parse()?;
        visitor.visit_map(access)
    }
    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let ret = visitor.visit_enum(VariantAccess::new(self));
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }
}

struct ArrayAccess<'de, 'a> {
    de: &'a mut MoleculeDeserializer<'de>,
    current_index: usize,
    count: usize,
    item_size: usize,
}

impl<'de, 'a> ArrayAccess<'de, 'a> {
    fn new(de: &'a mut MoleculeDeserializer<'de>, count: usize) -> Self {
        ArrayAccess {
            de,
            current_index: 0,
            count,
            item_size: 0,
        }
    }
    fn parse(&mut self) -> Result<()> {
        if self.count == 0 {
            return Err(Error::InvalidArray);
        }
        if self.de.data.len() % self.count == 0 {
            self.item_size = self.de.data.len() / self.count;
            Ok(())
        } else {
            Err(Error::InvalidArray)
        }
    }
}

impl<'de, 'a> de::SeqAccess<'de> for ArrayAccess<'de, 'a> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.current_index < self.count {
            let part = &self.de.data
                [self.current_index * self.item_size..(self.current_index + 1) * self.item_size];
            self.current_index += 1;
            let mut de = MoleculeDeserializer::new(part);
            let value = seed.deserialize(&mut de)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

struct FixvecAccess<'de, 'a> {
    de: &'a mut MoleculeDeserializer<'de>,
    current_index: usize,
    parts: Vec<&'de [u8]>,
}

impl<'de, 'a> FixvecAccess<'de, 'a> {
    fn new(de: &'a mut MoleculeDeserializer<'de>) -> Self {
        FixvecAccess {
            de,
            current_index: 0,
            parts: vec![],
        }
    }
    fn parse(&mut self) -> Result<()> {
        self.parts = disassemble_fixvec(self.de.data)?;
        Ok(())
    }
}

impl<'de, 'a> de::SeqAccess<'de> for FixvecAccess<'de, 'a> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.current_index < self.parts.len() {
            let part = self.parts[self.current_index];
            self.current_index += 1;
            let mut de = MoleculeDeserializer::new(part);
            let value = seed.deserialize(&mut de)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

struct TableAccess<'de, 'a> {
    de: &'a mut MoleculeDeserializer<'de>,
    current_index: usize,
    count: usize,
    parts: Vec<&'de [u8]>,
}

impl<'de, 'a> TableAccess<'de, 'a> {
    fn new(de: &'a mut MoleculeDeserializer<'de>, total_count: usize) -> Self {
        TableAccess {
            de,
            current_index: 0,
            count: total_count,
            parts: vec![],
        }
    }
    fn parse(&mut self) -> Result<()> {
        self.parts = disassemble_table(self.de.data)?;
        // TODO: compatible
        if self.parts.len() != self.count {
            return Err(Error::MismatchedTableFieldCount);
        }
        Ok(())
    }
}

impl<'de, 'a> de::MapAccess<'de> for TableAccess<'de, 'a> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.current_index < self.parts.len() {
            let de = U64Deserializer::<Error>::new(self.current_index as u64);
            Ok(Some(seed.deserialize(de)?))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        assert!(self.current_index < self.parts.len());
        let part = self.parts[self.current_index];
        self.current_index += 1;
        let mut de = MoleculeDeserializer::new(part);
        seed.deserialize(&mut de)
    }
}

struct VariantAccess<'de, 'a> {
    de: &'a mut MoleculeDeserializer<'de>,
}

impl<'de, 'a> VariantAccess<'de, 'a> {
    fn new(de: &'a mut MoleculeDeserializer<'de>) -> Self {
        VariantAccess { de }
    }
}

impl<'de, 'a> de::EnumAccess<'de> for VariantAccess<'de, 'a> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: de::DeserializeSeed<'de>,
    {
        todo!()
    }
}

impl<'de, 'a> de::VariantAccess<'de> for VariantAccess<'de, 'a> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        de::Deserialize::deserialize(self.de)
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(self.de, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_struct(self.de, "", fields, visitor)
    }
}

struct UnitVariantAccess<'de> {
    de: &'de mut MoleculeDeserializer<'de>,
}

impl<'de> UnitVariantAccess<'de> {
    fn new(de: &'de mut MoleculeDeserializer<'de>) -> Self {
        UnitVariantAccess { de }
    }
}

impl<'de> de::EnumAccess<'de> for UnitVariantAccess<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: de::DeserializeSeed<'de>,
    {
        todo!()
    }
}

impl<'de> de::VariantAccess<'de> for UnitVariantAccess<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }
}

//////////////////////////////////////////////////////////////////////////////
/// Deserialize an instance of type `T` from bytes of molecule.
///
pub fn from_slice<'a, T>(v: &'a [u8]) -> Result<T>
where
    T: de::Deserialize<'a>,
{
    let mut de = MoleculeDeserializer::new(v);
    let value = de::Deserialize::deserialize(&mut de)?;
    // de.end()?;
    Ok(value)
}
