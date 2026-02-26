use crate::{StringConverterFrom, StringConverterTo};
use chrono::NaiveDate;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S: Serializer>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&String::from_date(date))
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let dt = s.to_date().map_err(Error::custom)?;

    Ok(dt)
}
