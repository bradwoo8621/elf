use crate::serde::option_naive_datetime;
use crate::{BaseDataModel, Storable};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use std::collections::HashMap;
use watchmen_model_marco::adapt_model;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum SQLQueryPerformanceParsedQuerySegment {
    Str(String),
    Num(BigDecimal),
    Bool(bool),
    Map(HashMap<String, SQLQueryPerformanceParsedQuerySegment>),
    Vec(Vec<SQLQueryPerformanceParsedQuerySegment>),
}

#[adapt_model(storable)]
pub struct SQLQueryPerformance {
    pub id: Option<String>,
    pub query_text: Option<String>,
    pub query_spent: Option<BigDecimal>,
    #[serde(with = "option_naive_datetime")]
    pub query_date: Option<NaiveDateTime>,
    pub result_count: Option<i32>,
    pub query_parse: Option<HashMap<String, SQLQueryPerformanceParsedQuerySegment>>,
}
