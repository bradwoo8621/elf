use super::{serde_arc_string, serde_option_arc_string, serde_option_arc_topic_data_value};
use crate::{
    ActionExecuteLog, ArcTopicDataValue, ExecuteLogErrorStackTrace, ExecuteLogPrerequisiteDefinedAs,
};
use chrono::NaiveDateTime;
use elf_base::serde::naive_datetime;
use elf_model::{MonitorLogStatus, PipelineUnitId};
use std::sync::Arc;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitExecuteLog {
    #[serde(with = "serde_arc_string")]
    pub unit_id: Arc<PipelineUnitId>,
    #[serde(with = "serde_arc_string")]
    pub name: Arc<String>,
    /// definition of prerequisite
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite_defined_as: Option<ExecuteLogPrerequisiteDefinedAs>,

    pub status: MonitorLogStatus,
    /// keep none when step is ignored
    #[serde(with = "naive_datetime")]
    pub start_time: NaiveDateTime,
    /// keep 0 when step is ignored
    pub spent_in_mills: u32,
    /// if status is ERROR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ExecuteLogErrorStackTrace>,

    /// result of prerequisite, true when it is not defined
    pub prerequisite: bool,
    #[serde(with = "serde_option_arc_string")]
    pub loop_variable_name: Option<Arc<String>>,
    #[serde(with = "serde_option_arc_topic_data_value")]
    pub loop_variable_value: Option<Arc<ArcTopicDataValue>>,
    pub actions: Vec<ActionExecuteLog>,
}
