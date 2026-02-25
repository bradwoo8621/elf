use crate::{
    ActionDefinedAs, ActionTouchedValues, BaseDataModel, MonitorLogStatus, PipelineActionId,
    PipelineActionType, PrerequisiteDefinedAs, Storable,
};
use chrono::NaiveDateTime;
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct AlarmActionMonitorLog {
    pub action_id: Option<PipelineActionId>,
    pub r#type: Option<PipelineActionType>,
    /// definition of action
    pub defined_as: Option<ActionDefinedAs>,
    /// definition of prerequisite
    pub prerequisite_defined_as: Option<PrerequisiteDefinedAs>,

    pub status: Option<MonitorLogStatus>,
    /// keep none when step is ignored
    #[serde(with = "option_naive_datetime")]
    pub start_time: Option<NaiveDateTime>,
    /// keep 0 when step is ignored
    pub spent_in_mills: Option<u32>,
    /// if status is ERROR
    pub error: Option<String>,

    /// result of prerequisite, true when it is not defined
    pub prerequisite: Option<bool>,
    /// runtime action touched value
    pub touched: Option<ActionTouchedValues>,
    pub insert_count: Option<u32>,
    pub update_count: Option<u32>,
    pub delete_count: Option<u32>,
}
