use crate::{
    BaseDataModel, MonitorLogStatus, PipelineId, PipelineTopicData,
    PipelineTriggerTraceId, PrerequisiteDefinedAs, StageMonitorLog, Storable, TopicDataId, TopicId,
};
use chrono::NaiveDateTime;
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

pub type PipelineMonitorLogId = String;

#[adapt_model(storable)]
pub struct PipelineMonitorLog {
    pub uid: Option<PipelineMonitorLogId>,
    pub trace_id: Option<PipelineTriggerTraceId>,
    pub pipeline_id: Option<PipelineId>,
    pub topic_id: Option<TopicId>,
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
    pub data_id: Option<TopicDataId>,
    pub old_value: Option<PipelineTopicData>,
    pub new_value: Option<PipelineTopicData>,
    pub stages: Option<Vec<StageMonitorLog>>,
}
