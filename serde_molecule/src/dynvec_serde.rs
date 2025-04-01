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
    V: IntoIterator<Item = T> + Serialize,
{
    if core::any::type_name::<S>().contains("serde_molecule") {
        let parts: Result<Vec<_>, _> = value
            .into_iter()
            .map(|v| {
                to_vec(&v, false)
                    .map_err(|_| ser::Error::custom("failed to serialize element in vector"))
            })
            .collect();

        let parts = parts?; // Propagate error if any element fails to serialize.
        let data = assemble_table(&parts);
        serializer.serialize_bytes(&data)
    } else {
        value.serialize(serializer) // Use default serialization for others, e.g. serde_json
    }
}

pub fn deserialize<'de, D, T, V>(deserializer: D) -> Result<V, D::Error>
where
    D: Deserializer<'de>,
    V: FromIterator<T> + Deserialize<'de>,
    T: Deserialize<'de>,
{
    if core::any::type_name::<D>().contains("serde_molecule") {
        deserializer.deserialize_struct(
            DYNVEC_STR,
            &[],
            DynvecVisitor {
                marker: PhantomData,
                lifetime: PhantomData,
            },
        )
    } else {
        V::deserialize(deserializer)
    }
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
