use crate::serde::{naive_date, naive_datetime, naive_time};
use crate::{BaseDataModel, Storable};
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use watchmen_model_marco::adapt_model;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MonitorResultValue {
    Str(String),
    Num(BigDecimal),
    Bool(bool),
    #[serde(with = "naive_datetime")]
    DateTime(NaiveDateTime),
    #[serde(with = "naive_date")]
    Date(NaiveDate),
    #[serde(with = "naive_time")]
    Time(NaiveTime),
    Map(HashMap<String, MonitorResultValue>),
    Vec(Vec<MonitorResultValue>),
    None,
}

pub type MonitorResultValueMap = HashMap<String, MonitorResultValue>;
pub type MonitorResultValueVec = Vec<MonitorResultValue>;

#[adapt_model(storable)]
pub struct RawTopicMonitorResult {
    pub raw_topic_error_count: Option<i32>,
    pub raw_topic_error_details: Option<MonitorResultValueMap>,
}

#[adapt_model(storable)]
pub struct PipelineMonitorResult {
    pub error_count: Option<i32>,
    pub error_summary: Option<MonitorResultValueMap>,
    pub error_details: Option<MonitorResultValueVec>,
}

#[adapt_model(storable)]
pub struct MonitorResult {
    pub has_error: Option<bool>,
    pub raw_topic_error: Option<RawTopicMonitorResult>,
    pub pipeline_error: Option<PipelineMonitorResult>,
}
