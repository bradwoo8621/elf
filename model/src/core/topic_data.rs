use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use elf_base::serde::{naive_date, naive_datetime, naive_time};
use elf_base::DisplayLines;
use elf_model_marco::VariousValueTypes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

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

impl TopicDataValue {
    // noinspection DuplicatedCode
    pub fn map_to_display(map: &HashMap<String, TopicDataValue>) -> String {
        if map.is_empty() {
            return "Map[]".to_string();
        }

        let values_str = map
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .map(DisplayLines::indent)
            .collect::<Vec<String>>()
            .join(",\n");
        if values_str.is_empty() {
            "Map[]".to_string()
        } else {
            format!("Map[\n{}\n]", values_str)
        }
    }

    // noinspection DuplicatedCode
    pub fn vec_to_display(vec: &Vec<TopicDataValue>) -> String {
        if vec.is_empty() {
            return "Vec[]".to_string();
        }

        let values_str = vec
            .iter()
            .map(|value| format!("{}", value))
            .map(DisplayLines::indent)
            .collect::<Vec<String>>()
            .join(",\n");
        if values_str.is_empty() {
            "Vec[]".to_string()
        } else {
            format!("Vec[\n{}\n]", values_str)
        }
    }
}

impl Display for TopicDataValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Str(s) => write!(f, "Str[{}]", s),
            Self::Num(n) => write!(f, "Num[{}]", n),
            Self::Bool(b) => write!(f, "Bool[{}]", b),
            Self::DateTime(dt) => write!(f, "DateTime[{}]", dt),
            Self::Date(d) => write!(f, "Date[{}]", d),
            Self::Time(t) => write!(f, "Time[{}]", t),
            Self::Map(m) => {
                write!(f, "{}", Self::map_to_display(m))
            }
            Self::Vec(v) => {
                write!(f, "{}", Self::vec_to_display(v))
            }
            Self::None => write!(f, "None"),
        }
    }
}

pub type TopicData = HashMap<String, TopicDataValue>;
