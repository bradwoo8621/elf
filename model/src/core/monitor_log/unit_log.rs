use crate::{
    ActionMonitorLog, BaseDataModel, MonitorLogStatus, NotKnownYetDataStruct, PipelineUnitId,
    PrerequisiteDefinedAs, Storable,
};
use chrono::NaiveDateTime;
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

pub type UnitLoopVariableValue = NotKnownYetDataStruct;

#[adapt_model(storable)]
pub struct UnitMonitorLog {
    pub unit_id: Option<PipelineUnitId>,
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
    pub loop_variable_name: Option<String>,
    pub loop_variable_value: Option<UnitLoopVariableValue>,
    pub actions: Option<Vec<ActionMonitorLog>>,
}
