use crate::error;
use crate::ser::to_vec;
use serde::{Deserializer, Serialize, Serializer};

pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer<Ok = (), Error = error::Error>,
    T: Serialize,
{
    let data = to_vec(value, true)?;
    serializer.serialize_bytes(&data)
}

pub fn deserialize<'de, D, T>(_value: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
{
    todo!()
}
