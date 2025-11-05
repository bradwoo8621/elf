use super::BaseDataModel;
use serde_json::Value;
use std::collections::HashMap;

pub trait Pageable: BaseDataModel {
    fn page_number(&self) -> i32;
    fn page_size(&self) -> i32;
}

pub enum PageDataCell {
    Map(HashMap<String, Value>),
    Bdm(Box<dyn BaseDataModel>),
}

pub type PageDataRow = Vec<PageDataCell>;
pub type PageDataSet = Vec<PageDataRow>;

pub trait DataPage: Pageable {
    fn data(&self) -> PageDataSet;
    fn item_count(&self) -> i32;
    fn page_count(&self) -> i32;
}
