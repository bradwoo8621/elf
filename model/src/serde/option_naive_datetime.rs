use chrono::NaiveDateTime;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S: Serializer>(
    time: &Option<NaiveDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match time {
        Some(time) => serializer.serialize_str(&time.format("%Y-%m-%d %H:%M:%S").to_string()),
        _ => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        Ok(Some(
            NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").map_err(Error::custom)?,
        ))
    } else {
        Ok(None)
    }
}
