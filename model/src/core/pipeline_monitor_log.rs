use crate::{
	BaseDataModel, ModelErrorCode, Pageable, PipelineActionId, PipelineActionType, PipelineId,
	PipelineStageId, PipelineTriggerTraceId, PipelineUnitId, Storable, TenantId, TopicDataId,
	TopicId,
};
use chrono::NaiveDateTime;
use elf_base::serde::option_naive_datetime;
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
#[pattern = "upper-case"]
pub enum MonitorLogStatus {
    /// even step is ignored by prerequisite is false, it is treated as DONE
    DONE,
    /// step never be touched
    IGNORED,
    /// exception occurred
    ERROR,
}

/// TODO Any needs to be changed to some struct, according to where it is
///  there sure thing is, it is not a [String]
pub type NotKnownYetDataStruct = String;

#[adapt_model(storable)]
pub struct StandardMonitorLog {
    pub status: Option<MonitorLogStatus>,
    /// keep none when step is ignored
    #[serde(with = "option_naive_datetime")]
    pub start_time: Option<NaiveDateTime>,
    /// keep 0 when step is ignored
    pub spent_in_mills: Option<u32>,
    /// if status is ERROR
    pub error: Option<String>,
}

#[adapt_model(storable)]
pub struct ConditionalMonitorLog {
    //(StandardMonitorLog):
    /// result of prerequisite, True when it is not defined
    pub prerequisite: Option<bool>,
    /// definition of prerequisite
    pub prerequisite_defined_as: Option<NotKnownYetDataStruct>,
}

pub type MonitorLogActionId = String;

#[adapt_model(storable)]
pub struct MonitorLogAction {
    //(StandardMonitorLog):
    pub uid: Option<MonitorLogActionId>,
    pub action_id: Option<PipelineActionId>,
    pub r#type: Option<PipelineActionType>,
    pub insert_count: Option<u32>,
    pub update_count: Option<u32>,
    pub delete_count: Option<u32>,
    /// definition of action
    pub defined_as: Option<NotKnownYetDataStruct>,
    /// touched value,
    /// for deletion, update and insert, always be list of dict
    /// for read-exists, bool,
    /// for read-factor, no arithmetic, Any, depends on factor type
    /// for read-factor, arithmetic, Decimal
    /// for read-row, dict
    /// for read-rows, list of dict
    pub touched: Option<NotKnownYetDataStruct>,
}

#[adapt_model(storable)]
pub struct MonitorLogFindByAction {
    //(MonitorLogAction):
    /// runtime describing of find by
    pub find_by: Option<NotKnownYetDataStruct>,
}

#[adapt_model(storable)]
pub struct MonitorReadAction {
    //(MonitorLogFindByAction):
    pub r#type: Option<PipelineActionType>,
}

#[adapt_model(storable)]
pub struct MonitorWriteAction {
    //(MonitorLogFindByAction):
    pub r#type: Option<PipelineActionType>,
}

#[adapt_model(storable)]
pub struct MonitorDeleteAction {
    //(MonitorLogFindByAction):
    pub r#type: Option<PipelineActionType>,
}

#[adapt_model(storable)]
pub struct MonitorAlarmAction {
    //(MonitorLogAction, ConditionalMonitorLog):
    pub r#type: Option<PipelineActionType>,
}

#[adapt_model(storable)]
pub struct MonitorCopyToMemoryAction {
    //(MonitorLogAction):
    pub r#type: Option<PipelineActionType>,
}

#[adapt_model(storable)]
pub struct MonitorWriteToExternalAction {
    //(MonitorLogAction):
    pub r#type: Option<PipelineActionType>,
}

#[adapt_model(storable)]
pub struct MonitorLogUnit {
    //(ConditionalMonitorLog):
    pub unit_id: Option<PipelineUnitId>,
    pub name: Option<String>,
    pub loop_variable_name: Option<String>,
    pub loop_variable_value: Option<NotKnownYetDataStruct>,
    pub actions: Option<Vec<MonitorLogAction>>,
}

#[adapt_model(storable)]
pub struct MonitorLogStage {
    //(ConditionalMonitorLog):
    pub stage_id: Option<PipelineStageId>,
    pub name: Option<String>,
    pub units: Option<Vec<MonitorLogUnit>>,
}

pub type PipelineMonitorLogId = String;

#[adapt_model(storable)]
pub struct PipelineMonitorLog {
    //(ConditionalMonitorLog):
    pub uid: Option<PipelineMonitorLogId>,
    pub trace_id: Option<PipelineTriggerTraceId>,
    pub pipeline_id: Option<PipelineId>,
    pub topic_id: Option<TopicId>,
    pub data_id: Option<TopicDataId>,
    pub old_value: Option<NotKnownYetDataStruct>,
    pub new_value: Option<NotKnownYetDataStruct>,
    pub stages: Option<Vec<MonitorLogStage>>,
}

#[adapt_model(storable)]
pub struct PipelineMonitorLogCriteria {
    pub topic_id: Option<TopicId>,
    pub pipeline_id: Option<PipelineId>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub status: Option<MonitorLogStatus>,
    pub trace_id: Option<PipelineTriggerTraceId>,
    pub tenant_id: Option<TenantId>,
    /// [Pageable]
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
}

impl Pageable for PipelineMonitorLogCriteria {
    fn page_number(&self) -> u32 {
        if let Some(page_number) = self.page_number {
            page_number
        } else {
            1
        }
    }

    fn page_size(&self) -> u32 {
        if let Some(page_size) = self.page_size {
            page_size
        } else {
            20
        }
    }
}
