use crate::{BaseDataModel, Storable, TenantId, UserId};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use watchmen_base::serde::option_naive_datetime;
use watchmen_model_marco::{adapt_model, VariousValueTypes};

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
