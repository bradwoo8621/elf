use crate::{StringConverterFrom, StringConverterTo};
use chrono::NaiveDateTime;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S: Serializer>(
    datetime: &NaiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&String::from_datetime(datetime))
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let dt = s.to_datetime().map_err(Error::custom)?;

    Ok(dt)
}
