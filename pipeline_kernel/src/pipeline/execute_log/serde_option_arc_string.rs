use serde::{Deserialize, Deserializer, Serializer};
use std::ops::Deref;
use std::sync::Arc;

pub fn serialize<S: Serializer>(
    value: &Option<Arc<String>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(value) => serializer.serialize_str(value.deref()),
        _ => serializer.serialize_none(),
    }
}

#[allow(dead_code)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Arc<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        Ok(Some(Arc::new(s.to_owned())))
    } else {
        Ok(None)
    }
}
