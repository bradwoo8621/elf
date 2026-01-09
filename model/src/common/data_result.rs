use crate::{BaseDataModel, Storable};
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use elf_base::serde::{naive_date, naive_datetime, naive_time};
use elf_model_marco::{adapt_model, VariousValueTypes};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, VariousValueTypes)]
#[serde(untagged)]
pub enum DataResultSetCell {
    Str(String),
    Num(BigDecimal),
    Bool(bool),
    #[serde(with = "naive_datetime")]
    DateTime(NaiveDateTime),
    #[serde(with = "naive_date")]
    Date(NaiveDate),
    #[serde(with = "naive_time")]
    Time(NaiveTime),
    None,
}

pub type DataResultSetRow = Vec<DataResultSetCell>;
pub type DataResultSet = Vec<DataResultSetRow>;

#[adapt_model(storable)]
pub struct DataResult {
    pub columns: Option<Vec<String>>,
    pub data: Option<DataResultSet>,
}
