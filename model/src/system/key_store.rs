use crate::{BaseDataModel, Storable, TenantId, UserId};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use elf_base::serde::option_naive_datetime;
use elf_base::DisplayLines;
use elf_model_marco::{adapt_model, VariousValueTypes};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// various value types
#[derive(Serialize, Deserialize, Clone, Debug, VariousValueTypes)]
#[serde(untagged)]
pub enum KeyStoreValue {
    Str(String),
    Num(BigDecimal),
    Bool(bool),
    Map(HashMap<String, KeyStoreValue>),
    Vec(Vec<KeyStoreValue>),
}

impl KeyStoreValue {
    // noinspection DuplicatedCode
    pub fn map_to_display(map: &HashMap<String, KeyStoreValue>) -> String {
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
    pub fn vec_to_display(vec: &Vec<KeyStoreValue>) -> String {
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

impl Display for KeyStoreValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Str(s) => write!(f, "Str[{}]", s),
            Self::Num(n) => write!(f, "Num[{}]", n),
            Self::Bool(b) => write!(f, "Bool[{}]", b),
            Self::Map(m) => {
                write!(f, "{}", Self::map_to_display(m))
            }
            Self::Vec(v) => {
                write!(f, "{}", Self::vec_to_display(v))
            }
        }
    }
}

pub type KeyStoreParams = HashMap<String, KeyStoreValue>;

#[adapt_model(storable)]
pub struct KeyStore {
    pub tenant_id: Option<TenantId>,
    pub key_type: Option<String>,
    pub params: Option<KeyStoreParams>,
    #[serde(with = "option_naive_datetime")]
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<UserId>,
}

#[cfg(test)]
mod tests {
    use super::KeyStoreValue;

    #[test]
    fn test_add() {
        let s123 = serde_json::to_string(&KeyStoreValue::Str(String::from("123"))).unwrap();
        assert_eq!(s123, r#""123""#);

        let d123 = serde_json::from_str::<KeyStoreValue>("123").unwrap();
        assert!(matches!(d123, KeyStoreValue::Num(_)));
    }
}
