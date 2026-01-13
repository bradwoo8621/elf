use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::collections::HashMap;
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

pub type ArcTopicDataMap = HashMap<String, Arc<ArcTopicDataValue>>;
pub type ArcTopicData = Arc<ArcTopicDataMap>;
