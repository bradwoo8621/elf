use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use elf_base::{DisplayLines, StringConverterFrom};
use serde::ser::{Error, SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};
use serde_json::value::RawValue;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::Arc;

/// make everything [Arc].
#[derive(Debug, Clone)]
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

fn iterator_len_hint<I>(iter: &I) -> Option<usize>
where
    I: Iterator,
{
    match iter.size_hint() {
        (lo, Some(hi)) if lo == hi => Some(lo),
        _ => None,
    }
}

impl Serialize for ArcTopicDataValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_none(),
            Self::Str(s) => serializer.serialize_str(s),
            Self::Bool(b) => serializer.serialize_bool(*b),
            Self::Num(n) => {
                let value = RawValue::from_string(format!("{}", n)).map_err(S::Error::custom)?;
                value.serialize(serializer)
            }
            Self::DateTime(dt) => serializer.serialize_str(String::from_datetime(dt).as_str()),
            Self::Date(d) => serializer.serialize_str(String::from_date(d).as_str()),
            Self::Time(t) => serializer.serialize_str(String::from_time(t).as_str()),
            Self::Vec(vec) => {
                let mut iter = vec.deref().into_iter();
                let mut serializer = serializer.serialize_seq(iterator_len_hint(&iter))?;
                match iter.try_for_each(|item| serializer.serialize_element(item.deref())) {
                    Ok(val) => val,
                    Err(err) => return Err(err),
                }
                serializer.end()
            }
            Self::Map(map) => {
                let mut iter = map.deref().into_iter();
                let mut serializer = serializer.serialize_map(iterator_len_hint(&iter))?;
                match iter
                    .try_for_each(|(key, value)| serializer.serialize_entry(&key, value.deref()))
                {
                    Ok(val) => val,
                    Err(err) => return Err(err),
                }
                serializer.end()
            }
        }
    }
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

#[cfg(test)]
mod tests {
	use super::*;
	use bigdecimal::BigDecimal;
	use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
	use elf_base::StringConverterTo;
	use serde_json;

	#[test]
    fn test_serialize_none() {
        let value = ArcTopicDataValue::None;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "null");
    }

    #[test]
    fn test_serialize_bool_true() {
        let value = ArcTopicDataValue::Bool(true);
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "true");
    }

    #[test]
    fn test_serialize_bool_false() {
        let value = ArcTopicDataValue::Bool(false);
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "false");
    }

    #[test]
    fn test_serialize_string() {
        let value = ArcTopicDataValue::Str(Arc::new("hello".to_string()));
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"hello\"");
    }

    #[test]
    fn test_serialize_string_with_special_chars() {
        let value = ArcTopicDataValue::Str(Arc::new("hello\nworld".to_string()));
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"hello\\nworld\"");
    }

    #[test]
    fn test_serialize_number() {
        let value = ArcTopicDataValue::Num(Arc::new(BigDecimal::from(123)));
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "123");
    }

    #[test]
    fn test_serialize_number_decimal() {
        let value = ArcTopicDataValue::Num(Arc::new(BigDecimal::from(1234567890i64)));
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "1234567890");
    }

    #[test]
    fn test_serialize_datetime() {
        let dt = "2024-01-15 10:30:00".to_datetime().unwrap();
        let value = ArcTopicDataValue::DateTime(Arc::new(dt));
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"2024-01-15 10:30:00\"");
    }

    #[test]
    fn test_serialize_date() {
        let d = "2024-01-15".to_date().unwrap();
        let value = ArcTopicDataValue::Date(Arc::new(d));
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"2024-01-15\"");
    }

    #[test]
    fn test_serialize_time() {
        let t = "10:30:45".to_time().unwrap();
        let value = ArcTopicDataValue::Time(Arc::new(t));
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"10:30:45\"");
    }

    #[test]
    fn test_serialize_empty_vec() {
        let value = ArcTopicDataValue::Vec(Arc::new(vec![]));
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "[]");
    }

    #[test]
    fn test_serialize_vec_single_element() {
        let value = ArcTopicDataValue::Vec(Arc::new(vec![Arc::new(ArcTopicDataValue::Str(
            Arc::new("test".to_string()),
        ))]));
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "[\"test\"]");
    }

    #[test]
    fn test_serialize_vec_multiple_elements() {
        let value = ArcTopicDataValue::Vec(Arc::new(vec![
            Arc::new(ArcTopicDataValue::Str(Arc::new("a".to_string()))),
            Arc::new(ArcTopicDataValue::Num(Arc::new(BigDecimal::from(1)))),
            Arc::new(ArcTopicDataValue::Bool(true)),
        ]));
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "[\"a\",1,true]");
    }

    #[test]
    fn test_serialize_nested_vec() {
        let inner_vec = Arc::new(vec![Arc::new(ArcTopicDataValue::Str(Arc::new(
            "nested".to_string(),
        )))]);
        let value =
            ArcTopicDataValue::Vec(Arc::new(vec![Arc::new(ArcTopicDataValue::Vec(inner_vec))]));
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "[[\"nested\"]]");
    }

    #[test]
    fn test_serialize_empty_map() {
        let value = ArcTopicDataValue::Map(Arc::new(HashMap::new()));
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_serialize_map_single_entry() {
        let mut map = HashMap::new();
        map.insert(
            "key".to_string(),
            Arc::new(ArcTopicDataValue::Str(Arc::new("value".to_string()))),
        );
        let value = ArcTopicDataValue::Map(Arc::new(map));
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "{\"key\":\"value\"}");
    }

    #[test]
    fn test_serialize_map_multiple_entries() {
        let mut map = HashMap::new();
        map.insert(
            "name".to_string(),
            Arc::new(ArcTopicDataValue::Str(Arc::new("test".to_string()))),
        );
        map.insert(
            "count".to_string(),
            Arc::new(ArcTopicDataValue::Num(Arc::new(BigDecimal::from(42)))),
        );
        map.insert(
            "active".to_string(),
            Arc::new(ArcTopicDataValue::Bool(true)),
        );
        let value = ArcTopicDataValue::Map(Arc::new(map));
        let json = serde_json::to_string(&value).unwrap();
        assert!(json.contains("\"name\":\"test\""));
        assert!(json.contains("\"count\":42"));
        assert!(json.contains("\"active\":true"));
    }

    #[test]
    fn test_serialize_nested_map() {
        let mut inner_map = HashMap::new();
        inner_map.insert(
            "inner_key".to_string(),
            Arc::new(ArcTopicDataValue::Num(Arc::new(BigDecimal::from(1)))),
        );

        let mut outer_map = HashMap::new();
        outer_map.insert(
            "outer".to_string(),
            Arc::new(ArcTopicDataValue::Map(Arc::new(inner_map))),
        );

        let value = ArcTopicDataValue::Map(Arc::new(outer_map));
        let json = serde_json::to_string(&value).unwrap();
        assert!(json.contains("\"outer\":{"));
        assert!(json.contains("\"inner_key\":1"));
    }

    #[test]
    fn test_serialize_complex_nested() {
        let mut inner_map = HashMap::new();
        inner_map.insert(
            "value".to_string(),
            Arc::new(ArcTopicDataValue::Num(Arc::new(BigDecimal::from(100)))),
        );

        let vec_with_map = Arc::new(vec![Arc::new(ArcTopicDataValue::Map(Arc::new(inner_map)))]);

        let mut outer_map = HashMap::new();
        outer_map.insert(
            "items".to_string(),
            Arc::new(ArcTopicDataValue::Vec(vec_with_map)),
        );
        outer_map.insert(
            "name".to_string(),
            Arc::new(ArcTopicDataValue::Str(Arc::new("complex".to_string()))),
        );

        let value = ArcTopicDataValue::Map(Arc::new(outer_map));
        let json = serde_json::to_string(&value).unwrap();
        assert!(json.contains("\"name\":\"complex\""));
        assert!(json.contains("\"items\":["));
    }

    #[test]
    fn test_serialize_all_types_together() {
        let dt = NaiveDateTime::parse_from_str("2024-06-15 14:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let d = NaiveDate::parse_from_str("2024-06-15", "%Y-%m-%d").unwrap();
        let t = NaiveTime::parse_from_str("14:30:00", "%H:%M:%S").unwrap();

        let mut map = HashMap::new();
        map.insert(
            "str_field".to_string(),
            Arc::new(ArcTopicDataValue::Str(Arc::new("hello".to_string()))),
        );
        map.insert(
            "num_field".to_string(),
            Arc::new(ArcTopicDataValue::Num(Arc::new(BigDecimal::from(42)))),
        );
        map.insert(
            "bool_field".to_string(),
            Arc::new(ArcTopicDataValue::Bool(true)),
        );
        map.insert(
            "datetime_field".to_string(),
            Arc::new(ArcTopicDataValue::DateTime(Arc::new(dt))),
        );
        map.insert(
            "date_field".to_string(),
            Arc::new(ArcTopicDataValue::Date(Arc::new(d))),
        );
        map.insert(
            "time_field".to_string(),
            Arc::new(ArcTopicDataValue::Time(Arc::new(t))),
        );
        map.insert("null_field".to_string(), Arc::new(ArcTopicDataValue::None));

        let value = ArcTopicDataValue::Map(Arc::new(map));
        let json = serde_json::to_string(&value).unwrap();

        assert!(json.contains("\"str_field\":\"hello\""));
        assert!(json.contains("\"num_field\":42"));
        assert!(json.contains("\"bool_field\":true"));
        assert!(json.contains("\"datetime_field\":\"2024-06-15 14:30:00\""));
        assert!(json.contains("\"date_field\":\"2024-06-15\""));
        assert!(json.contains("\"time_field\":\"14:30:00\""));
        assert!(json.contains("\"null_field\":null"));
    }
}
