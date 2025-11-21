use crate::serde::option_naive_datetime;
use crate::{Auditable, BaseDataModel, Storable, TenantBasedTuple, TenantId, Tuple, UserId};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum OperationType {
    Create,
    Update,
    Delete,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
