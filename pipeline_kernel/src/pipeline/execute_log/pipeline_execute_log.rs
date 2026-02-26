use super::{serde_arc_string, serde_option_arc_topic_data};
use crate::{
    ArcTopicData, ExecuteLogErrorStackTrace, ExecuteLogPrerequisiteDefinedAs, StageExecuteLog,
};
use chrono::NaiveDateTime;
use elf_base::serde::naive_datetime;
use elf_model::{
    MonitorLogStatus, PipelineId, PipelineMonitorLogId, PipelineTriggerTraceId, TopicDataId,
    TopicId,
};
use std::sync::Arc;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineExecuteLog {
    pub uid: PipelineMonitorLogId,
    #[serde(with = "serde_arc_string")]
    pub trace_id: Arc<PipelineTriggerTraceId>,
    #[serde(with = "serde_arc_string")]
    pub pipeline_id: Arc<PipelineId>,
    #[serde(with = "serde_arc_string")]
    pub topic_id: Arc<TopicId>,
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
    #[serde(with = "serde_arc_string")]
    pub data_id: Arc<TopicDataId>,
    #[serde(with = "serde_option_arc_topic_data")]
    pub old_value: Option<ArcTopicData>,
    #[serde(with = "serde_option_arc_topic_data")]
    pub new_value: Option<ArcTopicData>,
    pub stages: Vec<StageExecuteLog>,
}
