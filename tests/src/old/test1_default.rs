use super::test1;
use lazy_static::lazy_static;
use molecule::{bytes::Bytes, prelude::*};
lazy_static! {
    pub static ref DEFAULT_BYTE: Byte = {
        let default = 0xcdu8;
        default.into()
    };
    pub static ref DEFAULT_U16: test1::U16 = {
        let default = 0xcdu16;
        test1::U16::new_unchecked(Bytes::copy_from_slice(&default.to_le_bytes()))
    };
    pub static ref DEFAULT_U32: test1::U32 = {
        let default = 0xcdu32;
        test1::U32::new_unchecked(Bytes::copy_from_slice(&default.to_le_bytes()))
    };
    pub static ref DEFAULT_U64: test1::U64 = {
        let default = 0xcdu64;
        test1::U64::new_unchecked(Bytes::copy_from_slice(&default.to_le_bytes()))
    };
    pub static ref DEFAULT_U128: test1::U128 = {
        let default = 0xcdu128;
        test1::U128::new_unchecked(Bytes::copy_from_slice(&default.to_le_bytes()))
    };
    pub static ref DEFAULT_FIXVEC: test1::Bytes = {
        let default: Vec<Byte> = vec![0xcd.into(), 0xcd.into(), 0xcd.into()];
        test1::Bytes::new_builder().extend(default).build()
    };
    pub static ref DEFAULT_DYNVEC: test1::BytesVector = {
        test1::BytesVector::new_builder()
            .push((*DEFAULT_FIXVEC).clone())
            .push((*DEFAULT_FIXVEC).clone())
            .push((*DEFAULT_FIXVEC).clone())
            .build()
    };
    pub static ref DEFAULT_STRUCT1: test1::Struct1 = {
        test1::Struct1::new_builder()
            .f1((*DEFAULT_BYTE).clone())
            .f2((*DEFAULT_U16).clone())
            .build()
    };
    pub static ref DEFAULT_ARRAY3: test1::Array3 = {
        test1::Array3::new_builder()
            .nth0((*DEFAULT_BYTE).clone())
            .nth1((*DEFAULT_BYTE).clone())
            .nth2((*DEFAULT_BYTE).clone())
            .build()
    };
    pub static ref DEFAULT_STRING: test1::String = {
        let default: Vec<Byte> = vec![('c' as u8).into(), ('d' as u8).into()];
        test1::String::new_builder().extend(default).build()
    };
    pub static ref DEFAULT_MAP: test1::Map = {
        let entry = test1::MapEntry::new_builder()
            .key((*DEFAULT_U128).clone())
            .value((*DEFAULT_FIXVEC).clone())
            .build();
        test1::Map::new_builder().push(entry).build()
    };
}
