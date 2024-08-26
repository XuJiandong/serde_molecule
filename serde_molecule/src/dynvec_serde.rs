use alloc::vec;
use alloc::vec::Vec;
use core::fmt;
use core::marker::PhantomData;

use crate::to_vec;
use crate::{de::DYNVEC_STR, molecule::assemble_table};
use serde::{
    de::{MapAccess, Visitor},
    ser, Deserialize, Deserializer, Serialize, Serializer,
};

pub fn serialize<T, S, V>(value: V, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
    V: IntoIterator<Item = T>,
{
    let mut parts = vec![];
    for v in value.into_iter() {
        // for dynvec, the element can't be molecule struct.
        parts.push(
            to_vec(&v, false)
                .map_err(|_| ser::Error::custom("failed to serialize element in vector"))?,
        );
    }
    let data = assemble_table(&parts);
    serializer.serialize_bytes(&data)
}

pub fn deserialize<'de, D, T, V>(deserializer: D) -> Result<V, D::Error>
where
    D: Deserializer<'de>,
    V: FromIterator<T>,
    T: Deserialize<'de>,
{
    deserializer.deserialize_struct(
        DYNVEC_STR,
        &[],
        DynvecVisitor {
            marker: PhantomData,
            lifetime: PhantomData,
        },
    )
}

struct DynvecVisitor<'de, T: Deserialize<'de>, V: FromIterator<T>> {
    marker: PhantomData<(T, V)>,
    lifetime: PhantomData<&'de ()>,
}

impl<'de, T: Deserialize<'de>, V: FromIterator<T>> Visitor<'de> for DynvecVisitor<'de, T, V> {
    type Value = V;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a dynvec")
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut result: Vec<T> = vec![];
        while map.next_key::<u64>()?.is_some() {
            result.push(map.next_value::<T>()?);
        }
        Ok(V::from_iter(result))
    }
}
