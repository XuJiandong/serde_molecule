use alloc::string::String;
use alloc::vec::Vec;

pub trait MoleculeAttribute {
    fn is_fixed_size() -> bool {
        true
    }
}

macro_rules! impl_attr {
    ($type:ty) => {
        impl MoleculeAttribute for $type {}
    };
    ($type:ty, $value:expr) => {
        impl MoleculeAttribute for $type {
            fn is_fixed_size() -> bool {
                $value
            }
        }
    };
}

impl_attr!(u8);
impl_attr!(i8);
impl_attr!(u16);
impl_attr!(i16);
impl_attr!(u32);
impl_attr!(i32);
impl_attr!(u64);
impl_attr!(i64);
impl_attr!(u128);
impl_attr!(i128);

impl<T> MoleculeAttribute for Vec<T> {
    fn is_fixed_size() -> bool {
        false
    }
}

impl MoleculeAttribute for &str {
    fn is_fixed_size() -> bool {
        false
    }
}

impl MoleculeAttribute for String {
    fn is_fixed_size() -> bool {
        false
    }
}

// TODO: more types
