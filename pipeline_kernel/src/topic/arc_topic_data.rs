use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use elf_base::DisplayLines;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

/// make every [Arc].
#[derive(Debug)]
pub enum ArcTopicDataValue {
    DateTime(Arc<NaiveDateTime>),
    Date(Arc<NaiveDate>),
    Time(Arc<NaiveTime>),
    Str(Arc<String>),
    Num(Arc<BigDecimal>),
    Bool(bool),
    Map(Arc<HashMap<String, Arc<ArcTopicDataValue>>>),
    Vec(Arc<Vec<Arc<ArcTopicDataValue>>>),
    None,
}

impl ArcTopicDataValue {
    // noinspection DuplicatedCode
    pub fn map_to_display(map: &HashMap<String, Arc<ArcTopicDataValue>>) -> String {
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
    pub fn vec_to_display(vec: &Vec<Arc<ArcTopicDataValue>>) -> String {
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

impl Display for ArcTopicDataValue {
    // noinspection DuplicatedCode
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

pub type ArcTopicDataMap = HashMap<String, Arc<ArcTopicDataValue>>;
pub type ArcTopicData = Arc<ArcTopicDataMap>;
