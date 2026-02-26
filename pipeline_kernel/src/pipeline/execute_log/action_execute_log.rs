use super::{serde_arc_action_type, serde_arc_string};
use crate::{ExecuteLogErrorStackTrace, ExecuteLogPrerequisiteDefinedAs};
use chrono::NaiveDateTime;
use elf_base::serde::naive_datetime;
use elf_model::{
    MapDataOnMonitorLog, MonitorLogDataValue, MonitorLogStatus, PipelineActionId,
    PipelineActionType,
};
use std::sync::Arc;

pub type ExecuteLogActionDefinedAs = String;
pub type ExecuteLogActionFindByCriteria = MapDataOnMonitorLog;
pub type ExecuteLogActionTouchedValues = MonitorLogDataValue;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionExecuteLog {
    #[serde(with = "serde_arc_string")]
    pub action_id: Arc<PipelineActionId>,
    #[serde(with = "serde_arc_action_type")]
    pub r#type: Arc<PipelineActionType>,
    /// definition of action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defined_as: Option<ExecuteLogActionDefinedAs>,
    /// definition of prerequisite
    /// - available only when action is [elf_model::AlarmAction]
    /// - otherwise leave it be [None]
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
    /// - available only when action is [elf_model::AlarmAction]
    /// - otherwise leave it be [None]
    pub prerequisite: bool,
    /// runtime describing of find by
    /// - available when action has find-by criteria, such as [elf_model::ReadFactorAction],
    /// - otherwise leave it be [None]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub find_by: Option<ExecuteLogActionFindByCriteria>,
    /// runtime action touched value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub touched: Option<ExecuteLogActionTouchedValues>,
    /// count of insert rows into storage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_count: Option<u32>,
    /// count of update rows into storage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_count: Option<u32>,
    /// count of delete rows into storage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_count: Option<u32>,
}
