//! Serialize a Rust data structure into molecule data.
// TODO: remove it
#![allow(unused)]

use crate::attribute::MoleculeAttribute;
use crate::error::{Error, Result};
use alloc::vec::Vec;
use serde::ser::{self, Serialize, SerializeTuple};

/// A structure for serializing Rust values into molecule.
pub struct MoleculeSerializer {
    //
    // The molecule format requires a header before the body. It should output
    // the body first, then the header. We can't gain any benefit from utilizing the
    // "Write" trait since it is sequential.
    data: Vec<u8>,

    //
    // true if the rust `struct` is mapping to molecule struct.
    // By default, all rust `struct` is mapping to molecule table.
    is_struct: bool,
}

impl MoleculeSerializer {
    /// Creates a new molecule serializer.    
    pub fn new(is_struct: bool) -> Self {
        MoleculeSerializer {
            data: vec![],
            is_struct,
        }
    }
}

impl MoleculeSerializer {
    pub fn to_vec(self) -> Vec<u8> {
        self.data
    }
    pub fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        self.data.extend(iter.into_iter());
    }
    pub fn is_struct(&self) -> bool {
        self.is_struct
    }
}

// dummy
pub struct Compound<'a> {
    ser: &'a mut MoleculeSerializer,
}

impl<'a> ser::Serializer for &'a mut MoleculeSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = FixVec<'a>;
    type SerializeTuple = Tuple<'a>;
    type SerializeStruct = Table<'a>;
    // not implemented
    type SerializeTupleStruct = Compound<'a>;
    type SerializeTupleVariant = Compound<'a>;
    type SerializeMap = Compound<'a>;
    type SerializeStructVariant = Compound<'a>;

    fn serialize_bool(self, value: bool) -> Result<()> {
        let value = match value {
            true => 1u8,
            false => 0u8,
        };
        self.data.push(value);
        Ok(())
    }

    fn serialize_i8(self, value: i8) -> Result<()> {
        let value = u8::from_le_bytes(value.to_le_bytes());
        self.data.push(value);
        Ok(())
    }

    fn serialize_i16(self, value: i16) -> Result<()> {
        let value = value.to_le_bytes();
        self.data.extend(value);
        Ok(())
    }

    fn serialize_i32(self, value: i32) -> Result<()> {
        let value = value.to_le_bytes();
        self.data.extend(value);
        Ok(())
    }

    fn serialize_i64(self, value: i64) -> Result<()> {
        let value = value.to_le_bytes();
        self.data.extend(value);
        Ok(())
    }

    fn serialize_i128(self, value: i128) -> Result<()> {
        let value = value.to_le_bytes();
        self.data.extend(value);
        Ok(())
    }

    fn serialize_u8(self, value: u8) -> Result<()> {
        self.data.push(value);
        Ok(())
    }

    fn serialize_u16(self, value: u16) -> Result<()> {
        let value = value.to_le_bytes();
        self.data.extend(value);
        Ok(())
    }

    fn serialize_u32(self, value: u32) -> Result<()> {
        let value = value.to_le_bytes();
        self.data.extend(value);
        Ok(())
    }

    fn serialize_u64(self, value: u64) -> Result<()> {
        let value = value.to_le_bytes();
        self.data.extend(value);
        Ok(())
    }

    fn serialize_u128(self, value: u128) -> Result<()> {
        let value = value.to_le_bytes();
        self.data.extend(value);
        Ok(())
    }

    fn serialize_f32(self, value: f32) -> Result<()> {
        let value = value.to_le_bytes();
        self.data.extend(value);
        Ok(())
    }

    fn serialize_f64(self, value: f64) -> Result<()> {
        let value = value.to_le_bytes();
        self.data.extend(value);
        Ok(())
    }

    fn serialize_char(self, value: char) -> Result<()> {
        self.data.extend(value.to_string().as_bytes());
        Ok(())
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        self.data.extend(value.as_bytes());
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        self.data.extend(value);
        Ok(())
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        unimplemented!()
    }

    /// Serialize newtypes without an object wrapper.

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(FixVec::new(self))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(Tuple::new(self))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        // TODO
        unimplemented!()
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        // In molecule struct, the inner fields must be molecule struct.
        Ok(Table::new(self, len, self.is_struct()))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

pub struct FixVec<'a> {
    ser: &'a mut MoleculeSerializer,
    parts: Vec<Vec<u8>>,
}

impl<'a> FixVec<'a> {
    pub fn new(ser: &'a mut MoleculeSerializer) -> Self {
        FixVec { ser, parts: vec![] }
    }
}

impl<'a> ser::SerializeSeq for FixVec<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.parts.push(to_vec(value, true)?);
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.ser.extend(assemble_table(self.parts));
        Ok(())
    }
}

// this tuple is used in serialization of [T; N]
pub struct Tuple<'a> {
    ser: &'a mut MoleculeSerializer,
    data: Vec<u8>,
}

impl<'a> Tuple<'a> {
    pub fn new(ser: &'a mut MoleculeSerializer) -> Self {
        Self { ser, data: vec![] }
    }
}

impl<'a> ser::SerializeTuple for Tuple<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let data = to_vec(value, true)?;
        self.data.extend(data);
        Ok(())
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

pub struct Table<'a> {
    ser: &'a mut MoleculeSerializer,
    parts: Vec<Vec<u8>>,
    len: usize,
    is_struct: bool,
}

impl<'a> Table<'a> {
    pub fn new(ser: &'a mut MoleculeSerializer, len: usize, is_struct: bool) -> Self {
        Table {
            ser,
            parts: vec![],
            len,
            is_struct,
        }
    }
}

impl<'a> ser::SerializeStruct for Table<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.parts.push(to_vec(value, self.is_struct)?);
        Ok(())
    }

    fn end(self) -> Result<()> {
        if self.is_struct {
            let data = assemble_struct(self.parts);
            self.ser.extend(data);
        } else {
            let data = assemble_table(self.parts);
            self.ser.extend(data);
        }
        Ok(())
    }
}

// assemble molecule table or dynvec
// https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0008-serialization/0008-serialization.md#table
// The table is a dynamic-size type. It can be considered as a dynvec but the length is fixed.
// The serializing steps are same as dynvec:
// Serialize the full size in bytes as a 32 bit unsigned integer in little-endian.
// Serialize all offset of fields as 32 bit unsigned integer in little-endian.
// Serialize all fields in it in the order they are declared.
//
pub fn assemble_table(parts: Vec<Vec<u8>>) -> Vec<u8> {
    let header_len = parts.len() + 1;
    let mut header = vec![0u32; header_len];
    let mut offset = (header_len * 4) as u32;
    for i in 1..header_len {
        header[i] = offset;
        offset += parts[i - 1].len() as u32;
    }
    header[0] = offset;
    let mut result = vec![];
    header
        .into_iter()
        .map(|u| u.to_le_bytes().to_vec())
        .fold(&mut result, |acc, item| {
            acc.extend(item);
            acc
        });
    parts.into_iter().fold(&mut result, |acc, item| {
        acc.extend(item);
        acc
    });
    result
}

// assemble molecule fixvec
// https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0008-serialization/0008-serialization.md#fixvec---fixed-vector
// There are two steps of serializing a fixvec:
// Serialize the length as a 32 bit unsigned integer in little-endian.
// Serialize all items in it.
pub fn assemble_fixvec(parts: Vec<Vec<u8>>) -> Vec<u8> {
    if parts.len() > 1 {
        let len = parts[0].len();
        for item in &parts {
            assert_eq!(item.len(), len);
        }
    }

    let mut result = vec![];
    let len = parts.len() as u32;
    result.extend(len.to_le_bytes());
    parts.into_iter().fold(&mut result, |acc, item| {
        acc.extend(item);
        acc
    });
    result
}
// assemble molecule struct
pub fn assemble_struct(parts: Vec<Vec<u8>>) -> Vec<u8> {
    let mut result = vec![];
    parts.into_iter().fold(&mut result, |acc, item| {
        acc.extend(item);
        acc
    });
    result
}

/// Serialize the given data structure as a molecule byte vector.
///
/// is_struct: is mapping to molecule struct or not.
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail
pub fn to_vec<T>(value: &T, is_struct: bool) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut serializer = MoleculeSerializer::new(is_struct);
    value.serialize(&mut serializer)?;
    Ok(serializer.to_vec())
}

impl<'a> ser::SerializeTupleStruct for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleVariant for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeMap for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStructVariant for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}
