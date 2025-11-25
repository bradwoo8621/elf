use crate::serde::{naive_date, naive_datetime, naive_time, option_naive_datetime};
use crate::{
    Auditable, BaseDataModel, DataSourceId, Factor, OptimisticLock, Storable, TenantBasedTuple,
    TenantId, Tuple, UserId,
};
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum, VariousValueTypes};

#[derive(Display, Serde, StrEnum)]
pub enum TopicKind {
    System,
    Business,
    Synonym,
}

#[derive(Display, Serde, StrEnum)]
pub enum TopicType {
    Raw,
    Meta,
    Distinct,
    Aggregate,
    Time,
    Ratio,
}

pub type TopicId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct Topic {
    pub topic_id: Option<TopicId>,
    pub name: Option<String>,
    pub r#type: Option<TopicType>,
    pub kind: Option<TopicKind>,
    pub data_source_id: Option<DataSourceId>,
    pub factors: Option<Vec<Factor>>,
    pub description: Option<String>,
}

/// the instance data id of topic
pub type TopicDataId = String;

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
