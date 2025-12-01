use chrono::NaiveDate;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S: Serializer>(time: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&time.format("%Y-%m-%d").to_string())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let dt = NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(Error::custom)?;

    Ok(dt)
}
