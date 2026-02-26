use crate::{StringConverterFrom, StringConverterTo};
use chrono::NaiveDate;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S: Serializer>(
    date: &Option<NaiveDate>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match date {
        Some(date) => serializer.serialize_str(&String::from_date(&date)),
        _ => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        Ok(Some(s.to_date().map_err(Error::custom)?))
    } else {
        Ok(None)
    }
}
