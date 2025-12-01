use chrono::NaiveTime;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S: Serializer>(time: &NaiveTime, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&time.format("%H:%M:%S").to_string())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let dt = NaiveTime::parse_from_str(s, "%H:%M:%S").map_err(Error::custom)?;

    Ok(dt)
}
