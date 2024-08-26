//! Serialize a Rust data structure into molecule data.
use crate::error::{Error, Result};
use crate::molecule::{assemble_fixvec, assemble_struct, assemble_table};
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use serde::ser::{self, Serialize};

/// Serialize the given data structure to byte vector.
///
/// Arguments
/// * is_struct - mapping to molecule struct. Set to false to map to molecule
/// table.
pub fn to_vec<T>(value: &T, is_struct: bool) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut serializer = MoleculeSerializer::new(is_struct);
    value.serialize(&mut serializer)?;
    Ok(serializer.into())
}

/// A structure for serializing Rust values into molecule.
pub(crate) struct MoleculeSerializer {
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

impl From<MoleculeSerializer> for Vec<u8> {
    fn from(value: MoleculeSerializer) -> Self {
        value.data
    }
}

impl MoleculeSerializer {
    pub fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        self.data.extend(iter.into_iter());
    }
    pub fn is_struct(&self) -> bool {
        self.is_struct
    }
}

// dummy
pub(crate) struct Compound<'a> {
    _ser: &'a mut MoleculeSerializer,
}

impl<'a> ser::Serializer for &'a mut MoleculeSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = FixVec<'a>;
    type SerializeTuple = Tuple<'a>;
    type SerializeStruct = Table<'a>;
    type SerializeMap = Map<'a>;
    // not implemented
    type SerializeTupleStruct = Compound<'a>;
    type SerializeTupleVariant = Compound<'a>;
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
        let len = value.len() as u32;
        self.data.extend(len.to_le_bytes());
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
        Err(Error::Unimplemented)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        self.serialize_u32(variant_index)
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
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_u32(variant_index)?;
        value.serialize(self)
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

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(FixVec::new(self))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(Tuple::new(self))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::Unimplemented)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::Unimplemented)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(Map::new(self))
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        // In molecule struct, the inner fields must be molecule struct.
        Ok(Table::new(self, len, self.is_struct()))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::Unimplemented)
    }
}

pub(crate) struct FixVec<'a> {
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
        self.ser.extend(assemble_fixvec(&self.parts)?);
        Ok(())
    }
}

// this tuple is used in serialization of [T; N]
pub(crate) struct Tuple<'a> {
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
        self.ser.extend(self.data);
        Ok(())
    }
}

pub(crate) struct Table<'a> {
    ser: &'a mut MoleculeSerializer,
    parts: Vec<Vec<u8>>,
    count: usize,
    is_struct: bool,
}

impl<'a> Table<'a> {
    pub fn new(ser: &'a mut MoleculeSerializer, count: usize, is_struct: bool) -> Self {
        Table {
            ser,
            parts: vec![],
            count,
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
        if self.parts.len() != self.count {
            return Err(Error::InvalidTableCount);
        }
        if self.is_struct {
            let data = assemble_struct(self.parts);
            self.ser.extend(data);
        } else {
            let data = assemble_table(&self.parts);
            self.ser.extend(data);
        }
        Ok(())
    }
}

pub(crate) struct Map<'a> {
    ser: &'a mut MoleculeSerializer,
    parts: Vec<Vec<u8>>,
    temp_key: Vec<u8>,
}

impl<'a> Map<'a> {
    pub fn new(ser: &'a mut MoleculeSerializer) -> Self {
        Self {
            ser,
            parts: vec![],
            temp_key: vec![],
        }
    }
}

impl<'a> ser::SerializeMap for Map<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let mut ser = MoleculeSerializer::new(false);
        let result = key.serialize(&mut ser);
        self.temp_key = ser.into();
        result
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let mut ser = MoleculeSerializer::new(false);
        let result = value.serialize(&mut ser);
        let parts = vec![self.temp_key.clone(), ser.into()];
        self.parts.push(assemble_table(&parts));
        result
    }

    fn end(self) -> Result<()> {
        self.ser.extend(assemble_table(&self.parts));
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Unimplemented)
    }

    fn end(self) -> Result<()> {
        Err(Error::Unimplemented)
    }
}

impl<'a> ser::SerializeTupleVariant for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Unimplemented)
    }

    fn end(self) -> Result<()> {
        Err(Error::Unimplemented)
    }
}

impl<'a> ser::SerializeStructVariant for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Unimplemented)
    }

    fn end(self) -> Result<()> {
        Err(Error::Unimplemented)
    }
}
