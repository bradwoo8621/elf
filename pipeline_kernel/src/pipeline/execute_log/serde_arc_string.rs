use serde::{Deserialize, Deserializer, Serializer};
use std::ops::Deref;
use std::sync::Arc;

pub fn serialize<S: Serializer>(value: &Arc<String>, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(value.deref())
}

#[allow(dead_code)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<Arc<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(Arc::new(s.to_owned()))
}
