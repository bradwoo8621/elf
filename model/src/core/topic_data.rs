use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use watchmen_base::serde::{naive_date, naive_datetime, naive_time};
use watchmen_base::DisplayLines;
use watchmen_model_marco::VariousValueTypes;

/// the instance data id of topic
pub type TopicDataId = String;

/// apart from numbers and booleans, values will be preferentially matched against strings
/// rather than attempting to match various date/time formats.
#[derive(Deserialize, Serialize, Clone, Debug, VariousValueTypes)]
#[serde(untagged)]
pub enum TopicDataValue {
    Str(String),
    Num(BigDecimal),
    Bool(bool),
    #[serde(with = "naive_datetime")]
    DateTime(NaiveDateTime),
    #[serde(with = "naive_date")]
    Date(NaiveDate),
    #[serde(with = "naive_time")]
    Time(NaiveTime),
    Map(HashMap<String, TopicDataValue>),
    Vec(Vec<TopicDataValue>),
    None,
}

impl Display for TopicDataValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Str(s) => write!(f, "{}", s),
            Self::Num(n) => write!(f, "{}", n),
            Self::Bool(b) => write!(f, "{}", b),
            Self::DateTime(dt) => write!(f, "{}", dt),
            Self::Date(d) => write!(f, "{}", d),
            Self::Time(t) => write!(f, "{}", t),
            Self::Map(m) => {
                let values_str = m
                    .iter()
                    .map(|(key, value)| format!("{}={}", key, value))
                    .map(DisplayLines::indent)
                    .collect::<Vec<String>>()
                    .join(",\n");
                if values_str.is_empty() {
                    write!(f, "HashMap[]")
                } else {
                    write!(f, "HashMap[\n{}\n]", values_str)
                }
            }
            Self::Vec(v) => {
                let values_str = v
                    .iter()
                    .map(|value| format!("{}", value))
                    .map(DisplayLines::indent)
                    .collect::<Vec<String>>()
                    .join(",\n");
                if values_str.is_empty() {
                    write!(f, "Vec[]")
                } else {
                    write!(f, "Vec[\n{}\n]", values_str)
                }
            }
            Self::None => Ok(()),
        }
    }
}

pub type TopicData = HashMap<String, TopicDataValue>;
