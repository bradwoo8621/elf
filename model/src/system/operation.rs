use crate::{
    Auditable, BaseDataModel, ModelErrorCode, Storable, TenantBasedTuple, TenantId, Tuple, UserId,
};
use bigdecimal::BigDecimal;
use elf_base::serde::option_naive_datetime;
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum, VariousValueTypes};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Display, Serde, StrEnum)]
pub enum OperationType {
    Create,
    Update,
    Delete,
}

#[derive(Serialize, Deserialize, Clone, Debug, VariousValueTypes)]
#[serde(untagged)]
pub enum OperationContent {
    Str(String),
    Num(BigDecimal),
    Bool(bool),
    Map(HashMap<String, OperationContent>),
    Vec(Vec<OperationContent>),
}

#[adapt_model(tenant_based)]
pub struct Operation {
    pub record_id: Option<String>,
    pub operation_type: Option<String>,
    pub tuple_key: Option<String>,
    pub tuple_type: Option<String>,
    pub tuple_id: Option<String>,
    pub content: Option<HashMap<String, OperationContent>>,
    pub version_num: Option<String>,
}
