use crate::{
    BaseDataModel, MonitorLogStatus, PipelineStageId, PrerequisiteDefinedAs, Storable,
    UnitMonitorLog,
};
use chrono::NaiveDateTime;
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct StageMonitorLog {
    pub stage_id: Option<PipelineStageId>,
    pub name: Option<String>,
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
    pub units: Option<Vec<UnitMonitorLog>>,
}
