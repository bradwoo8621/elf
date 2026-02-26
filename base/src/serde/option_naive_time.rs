use crate::{StringConverterFrom, StringConverterTo};
use chrono::NaiveTime;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S: Serializer>(
    time: &Option<NaiveTime>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match time {
        Some(time) => serializer.serialize_str(&String::from_time(time)),
        _ => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        Ok(Some(s.to_time().map_err(Error::custom)?))
    } else {
        Ok(None)
    }
}
