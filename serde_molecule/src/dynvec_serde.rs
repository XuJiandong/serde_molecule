use crate::error;
use crate::{ser::assemble_table, to_vec};
use serde::{Deserializer, Serialize, Serializer};

pub fn serialize<T, S>(value: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer<Ok = (), Error = error::Error>,
    T: Serialize,
{
    let mut parts = vec![];
    for v in value {
        // for dynvec, the element can't be molecule struct.
        parts.push(to_vec(v, false)?);
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
