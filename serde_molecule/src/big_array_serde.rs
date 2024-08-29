// from https://github.com/est31/serde-big-array/blob/master/src/const_generics.rs

use alloc::vec::Vec;
use core::fmt;
use core::marker::PhantomData;
use core::result;
use serde::de::{Deserialize, Deserializer, Error, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeTuple, Serializer};

pub trait BigArray<'de, T>: Sized {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize;
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>;
}

impl<'de, T, const N: usize> BigArray<'de, T> for [T; N] {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        let mut seq = serializer.serialize_tuple(self.len())?;
        for elem in &self[..] {
            seq.serialize_element(elem)?;
        }
        seq.end()
    }

    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        struct ArrayVisitor<T> {
            element: PhantomData<T>,
        }

        impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<[T; N]>
        where
            T: Deserialize<'de>,
        {
            type Value = [T; N];

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "an array of length {}", N)
            }

            fn visit_seq<A>(self, mut seq: A) -> result::Result<[T; N], A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec = Vec::with_capacity(N);
                for i in 0..N {
                    let val = seq
                        .next_element()?
                        .ok_or_else(|| Error::invalid_length(i, &self))?;
                    vec.push(val);
                }
                vec.try_into()
                    .map_err(|_| Error::custom("Failed to convert Vec to array"))
            }
        }

        let visitor = ArrayVisitor {
            element: PhantomData,
        };
        // The allow is needed to support (32 + 33) like expressions
        #[allow(unused_parens)]
        deserializer.deserialize_tuple(N, visitor)
    }
}

pub fn serialize<'de, T, S, E>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    E: Serialize,
    T: BigArray<'de, E>,
{
    BigArray::serialize(value, serializer)
}

pub fn deserialize<'de, D, T, E>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    E: Deserialize<'de>,
    T: BigArray<'de, E>,
{
    BigArray::deserialize(deserializer)
}
