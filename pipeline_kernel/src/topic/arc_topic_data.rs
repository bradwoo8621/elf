use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use elf_base::DisplayLines;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
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

pub trait ArcFrom<T>: Sized + From<T> {
    fn wrap(value: Arc<T>) -> Arc<Self>;
    fn arc_from(value: T) -> Arc<Self>;
}

impl ArcFrom<NaiveDateTime> for ArcTopicDataValue {
    fn wrap(value: Arc<NaiveDateTime>) -> Arc<Self> {
        Arc::new(Self::DateTime(value))
    }

    fn arc_from(value: NaiveDateTime) -> Arc<Self> {
        Arc::new(Self::from(value))
    }
}

impl From<NaiveDateTime> for ArcTopicDataValue {
    fn from(value: NaiveDateTime) -> Self {
        Self::DateTime(Arc::new(value))
    }
}

impl ArcFrom<NaiveDate> for ArcTopicDataValue {
    fn wrap(value: Arc<NaiveDate>) -> Arc<Self> {
        Arc::new(Self::Date(value))
    }

    fn arc_from(value: NaiveDate) -> Arc<Self> {
        Arc::new(Self::from(value))
    }
}

impl From<NaiveDate> for ArcTopicDataValue {
    fn from(value: NaiveDate) -> Self {
        Self::Date(Arc::new(value))
    }
}

impl ArcFrom<NaiveTime> for ArcTopicDataValue {
    fn wrap(value: Arc<NaiveTime>) -> Arc<Self> {
        Arc::new(Self::Time(value))
    }

    fn arc_from(value: NaiveTime) -> Arc<Self> {
        Arc::new(Self::from(value))
    }
}

impl From<NaiveTime> for ArcTopicDataValue {
    fn from(value: NaiveTime) -> Self {
        Self::Time(Arc::new(value))
    }
}

impl ArcFrom<String> for ArcTopicDataValue {
    fn wrap(value: Arc<String>) -> Arc<Self> {
        Arc::new(Self::Str(value))
    }

    fn arc_from(value: String) -> Arc<Self> {
        Arc::new(Self::from(value))
    }
}

impl From<String> for ArcTopicDataValue {
    fn from(value: String) -> Self {
        Self::Str(Arc::new(value))
    }
}

impl ArcFrom<BigDecimal> for ArcTopicDataValue {
    fn wrap(value: Arc<BigDecimal>) -> Arc<Self> {
        Arc::new(Self::Num(value))
    }

    fn arc_from(value: BigDecimal) -> Arc<Self> {
        Arc::new(Self::from(value))
    }
}

impl From<BigDecimal> for ArcTopicDataValue {
    fn from(value: BigDecimal) -> Self {
        Self::Num(Arc::new(value))
    }
}

impl ArcFrom<bool> for ArcTopicDataValue {
    fn wrap(value: Arc<bool>) -> Arc<Self> {
        Arc::new(Self::Bool(*value.deref()))
    }

    fn arc_from(value: bool) -> Arc<Self> {
        Arc::new(Self::from(value))
    }
}

impl From<bool> for ArcTopicDataValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl ArcFrom<HashMap<String, Arc<ArcTopicDataValue>>> for ArcTopicDataValue {
    fn wrap(value: Arc<HashMap<String, Arc<ArcTopicDataValue>>>) -> Arc<Self> {
        Arc::new(Self::Map(value))
    }

    fn arc_from(value: HashMap<String, Arc<ArcTopicDataValue>>) -> Arc<Self> {
        Arc::new(ArcTopicDataValue::from(value))
    }
}

impl From<HashMap<String, Arc<ArcTopicDataValue>>> for ArcTopicDataValue {
    fn from(value: HashMap<String, Arc<ArcTopicDataValue>>) -> Self {
        ArcTopicDataValue::Map(Arc::new(value))
    }
}

impl ArcFrom<Vec<Arc<ArcTopicDataValue>>> for ArcTopicDataValue {
    fn wrap(value: Arc<Vec<Arc<ArcTopicDataValue>>>) -> Arc<Self> {
        Arc::new(Self::Vec(value))
    }

    fn arc_from(value: Vec<Arc<ArcTopicDataValue>>) -> Arc<Self> {
        Arc::new(ArcTopicDataValue::from(value))
    }
}

impl From<Vec<Arc<ArcTopicDataValue>>> for ArcTopicDataValue {
    fn from(value: Vec<Arc<ArcTopicDataValue>>) -> Self {
        ArcTopicDataValue::Vec(Arc::new(value))
    }
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
            Self::Num(n) => write!(f, "Num[{}]", n.to_plain_string()),
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
