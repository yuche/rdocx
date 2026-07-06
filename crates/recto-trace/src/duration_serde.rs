use serde::{Deserialize, Deserializer, Serializer};
use std::time::Duration;

pub fn serialize<S: Serializer>(d: &Duration, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u64(d.as_nanos() as u64)
}

pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
    let nanos = u64::deserialize(d)?;
    Ok(Duration::from_nanos(nanos))
}
