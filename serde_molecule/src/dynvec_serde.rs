use crate::molecule::assemble_table;
use crate::to_vec;
use serde::{Deserializer, Serialize, Serializer};

pub fn serialize<T, S, V>(value: V, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
    V: IntoIterator<Item = T>,
{
    let mut parts = vec![];
    for v in value.into_iter() {
        // for dynvec, the element can't be molecule struct.
        parts.push(to_vec(&v, false).unwrap());
    }
    let data = assemble_table(parts);
    serializer.serialize_bytes(&data)
}

pub fn deserialize<'de, D, T>(_value: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
{
    todo!()
}
