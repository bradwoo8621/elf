use crate::serde::option_naive_datetime;
use crate::{BaseDataModel, Storable, TenantId, UserId};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use std::collections::HashMap;
use watchmen_model_marco::adapt_model;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum KeyStoreValue {
    Str(String),
    Num(BigDecimal),
    Bool(bool),
    Map(HashMap<String, KeyStoreValue>),
    Vec(Vec<KeyStoreValue>),
}

#[adapt_model(storable)]
pub struct KeyStore {
    pub tenant_id: Option<TenantId>,
    pub key_type: Option<String>,
    pub params: Option<HashMap<String, KeyStoreValue>>,
    #[serde(with = "option_naive_datetime")]
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<UserId>,
}
