use crate::BaseDataModel;
use elf_model_marco::VariousValueTypes;
use serde_json::Value;
use std::collections::HashMap;

pub trait Pageable: BaseDataModel {
    fn page_number(&self) -> u32;
    fn page_size(&self) -> u32;
}

/// TODO how to apply serde?
#[derive(VariousValueTypes)]
pub enum PageDataCell {
    Map(HashMap<String, Value>),
    Bdm(Box<dyn BaseDataModel>),
}

pub type PageDataRow = Vec<PageDataCell>;
pub type PageDataSet = Vec<PageDataRow>;

pub trait DataPage: Pageable {
    fn data(&self) -> PageDataSet;
    fn item_count(&self) -> u32;
    fn page_count(&self) -> u32;
}
