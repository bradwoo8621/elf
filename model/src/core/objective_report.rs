use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, LastVisit, ObjectiveId, ObjectiveTimeframe, ObjectiveVariable,
    Storable, TenantId, UserBasedTuple, UserId,
};
use bigdecimal::BigDecimal;
use std::collections::HashMap;
use watchmen_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct Variable {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[adapt_model(storable)]
pub struct ObjectiveCellValue {
    pub current_value: Option<BigDecimal>,
    pub previous_value: Option<BigDecimal>,
    pub chain_value: Option<BigDecimal>,
    pub failed: Option<bool>,
}

pub type ObjectiveReportCellTargetId = String;

#[adapt_model(storable)]
pub struct ObjectiveCellTarget {
    pub target_id: Option<ObjectiveReportCellTargetId>,
    pub name: Option<String>,
    pub row: Option<i32>,
    pub cell: Option<i32>,
    pub objective_id: Option<ObjectiveId>,
    pub value: Option<ObjectiveCellValue>,
    /// TODO don't know the exact type, since in python, it is [Dict]
    pub parameters: Option<HashMap<String, String>>,
}

pub type ObjectiveReportId = String;

#[adapt_model(user_based, audit, last_visit)]
pub struct ObjectiveReport {
    pub objective_report_id: Option<ObjectiveReportId>,
    pub name: Option<String>,
    pub variables: Option<Vec<ObjectiveVariable>>,
    pub time_frame: Option<ObjectiveTimeframe>,
    pub cells: Option<Vec<ObjectiveCellTarget>>,
}
