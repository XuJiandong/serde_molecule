//! Deserialize molecule data to a Rust data structure.
use crate::{
    error::{Error, Result},
    molecule::{disassemble_fixvec, disassemble_table, unpack_number},
    struct_serde::MoleculeStructDeserializer,
};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use serde::de::{self, value::U64Deserializer};

//////////////////////////////////////////////////////////////////////////////
pub(crate) const DYNVEC_STR: &str = "$serde_molecule::DynVec";

//////////////////////////////////////////////////////////////////////////////
/// Deserialize an instance of type `T` from bytes of molecule.
///
/// Arguments
/// * is_struct - mapping to molecule struct. Set to false to map to molecule table.
pub fn from_slice<'a, T>(v: &'a [u8], is_struct: bool) -> Result<T>
where
    T: de::Deserialize<'a>,
{
    if is_struct {
        let mut de = MoleculeStructDeserializer::new(v.to_vec());
        let value = de::Deserialize::deserialize(&mut de)?;
        Ok(value)
    } else {
        let mut de = MoleculeDeserializer::new(v);
        let value = de::Deserialize::deserialize(&mut de)?;
        Ok(value)
    }
}

/// A structure that deserializes molecule into Rust values.
pub struct MoleculeDeserializer<'de> {
    data: &'de [u8],
}

impl<'de> MoleculeDeserializer<'de> {
    pub fn new(data: &'de [u8]) -> Self {
        MoleculeDeserializer { data }
    }
}

macro_rules! as_primitive {
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
    as_primitive!(as_u8, u8, 1);
    as_primitive!(as_u16, u16, 2);
    as_primitive!(as_u32, u32, 4);
    as_primitive!(as_u64, u64, 8);
    as_primitive!(as_u128, u128, 16);
    as_primitive!(as_i8, i8, 1);
    as_primitive!(as_i16, i16, 2);
    as_primitive!(as_i32, i32, 4);
    as_primitive!(as_i64, i64, 8);
    as_primitive!(as_i128, i128, 16);
    as_primitive!(as_f32, f32, 4);
    as_primitive!(as_f64, f64, 8);
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

impl<'de> de::Deserializer<'de> for &mut MoleculeDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
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
        let value = self.as_u32()?;
        match char::from_u32(value) {
            Some(ch) => visitor.visit_char(ch),
            None => Err(Error::InvalidChar),
        }
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
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut access = TableAccess::new(self, len);
        access.parse()?;
        visitor.visit_seq(access)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut access = MappingAccess::new(self);
        access.parse()?;
        visitor.visit_map(access)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if name == DYNVEC_STR {
            let mut access = DynvecAccess::new(self);
            access.parse()?;
            visitor.visit_map(access)
        } else {
            let mut access = TableAccess::new(self, fields.len());
            access.parse()?;
            visitor.visit_map(access)
        }
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
        visitor.visit_enum(UnionAccess::new(self))
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
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
    pub fn new(de: &'a mut MoleculeDeserializer<'de>, count: usize) -> Self {
        ArrayAccess {
            de,
            current_index: 0,
            count,
            item_size: 0,
        }
    }
    pub fn parse(&mut self) -> Result<()> {
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

impl<'de> de::SeqAccess<'de> for ArrayAccess<'de, '_> {
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

impl<'de> de::SeqAccess<'de> for FixvecAccess<'de, '_> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.current_index < self.parts.len() {
            let part = self.parts[self.current_index];
            self.current_index += 1;
            // elements in fixvec should be fixed(struct, primitive type, etc)
            let mut de = MoleculeStructDeserializer::new(part.to_vec());
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
    fn new(de: &'a mut MoleculeDeserializer<'de>, count: usize) -> Self {
        TableAccess {
            de,
            current_index: 0,
            count,
            parts: vec![],
        }
    }
    fn parse(&mut self) -> Result<()> {
        self.parts = disassemble_table(self.de.data)?;
        // always enable compatible for molecule table
        if self.parts.len() < self.count {
            return Err(Error::MismatchedTableFieldCount);
        }
        Ok(())
    }
}

impl<'de> de::MapAccess<'de> for TableAccess<'de, '_> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.current_index < self.count {
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

// used for tuple struct
impl<'de> de::SeqAccess<'de> for TableAccess<'de, '_> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        Ok(Some(de::MapAccess::next_value_seed(self, seed)?))
    }
}

struct MappingAccess<'de, 'a> {
    de: &'a mut MoleculeDeserializer<'de>,
    current_index: usize,
    parts: Vec<(&'de [u8], &'de [u8])>,
}

impl<'de, 'a> MappingAccess<'de, 'a> {
    fn new(de: &'a mut MoleculeDeserializer<'de>) -> Self {
        MappingAccess {
            de,
            current_index: 0,
            parts: vec![],
        }
    }
    fn parse(&mut self) -> Result<()> {
        let all = disassemble_table(self.de.data)?;
        for item in all.into_iter() {
            let kv = disassemble_table(item)?;
            if kv.len() != 2 {
                return Err(Error::InvalidMap);
            }
            self.parts.push((kv[0], kv[1]));
        }
        Ok(())
    }
}

impl<'de> de::MapAccess<'de> for MappingAccess<'de, '_> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.current_index < self.parts.len() {
            let key_slice = self.parts[self.current_index].0;
            let mut de = MoleculeDeserializer::new(key_slice);
            Ok(Some(seed.deserialize(&mut de)?))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        assert!(self.current_index < self.parts.len());
        let value_slice = self.parts[self.current_index].1;
        self.current_index += 1;
        let mut de = MoleculeDeserializer::new(value_slice);
        seed.deserialize(&mut de)
    }
}

struct DynvecAccess<'de, 'a> {
    de: &'a mut MoleculeDeserializer<'de>,
    current_index: usize,
    parts: Vec<&'de [u8]>,
}

impl<'de, 'a> DynvecAccess<'de, 'a> {
    fn new(de: &'a mut MoleculeDeserializer<'de>) -> Self {
        DynvecAccess {
            de,
            current_index: 0,
            parts: vec![],
        }
    }
    fn parse(&mut self) -> Result<()> {
        self.parts = disassemble_table(self.de.data)?;
        Ok(())
    }
}

impl<'de> de::MapAccess<'de> for DynvecAccess<'de, '_> {
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

struct UnionAccess<'de, 'a> {
    de: &'a mut MoleculeDeserializer<'de>,
}

impl<'de, 'a> UnionAccess<'de, 'a> {
    fn new(de: &'a mut MoleculeDeserializer<'de>) -> Self {
        UnionAccess { de }
    }
}

impl<'de> de::EnumAccess<'de> for UnionAccess<'de, '_> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let id = unpack_number(self.de.data, 0)?;
        self.de.data = &self.de.data[4..];
        let de = U64Deserializer::<Error>::new(id as u64);
        Ok((seed.deserialize(de)?, self))
    }
}

impl<'de> de::VariantAccess<'de> for UnionAccess<'de, '_> {
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

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_tuple_struct(self.de, "", len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_struct(self.de, "", fields, visitor)
    }
}
