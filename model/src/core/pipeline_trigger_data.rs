use crate::{
    BaseDataModel, PipelineTriggerType, Storable, TenantId, TopicCode, TopicData, TopicDataId,
};
use elf_model_marco::adapt_model;

pub type PipelineTriggerTraceId = String;

#[adapt_model(storable)]
pub struct PipelineTriggerData {
    /// topic name
    pub code: Option<TopicCode>,
    /// current data
    pub data: Option<TopicData>,
    pub trigger_type: Option<PipelineTriggerType>,
    /// pass tenant id when use super admin
    pub tenant_id: Option<TenantId>,
    /// user given trace id, typically leave it as none
    pub trace_id: Option<PipelineTriggerTraceId>,
}

#[adapt_model(storable)]
pub struct PipelineTriggerDataWithPAT {
    pub pat: Option<String>,
    /// [PipelineTriggerData]
    /// topic name
    pub code: Option<String>,
    /// current data
    pub data: Option<TopicData>,
    pub trigger_type: Option<PipelineTriggerType>,
    /// pass tenant id when use super admin
    pub tenant_id: Option<TenantId>,
    /// user given trace id, typically leave it as none
    pub trace_id: Option<PipelineTriggerTraceId>,
}

#[adapt_model(storable)]
pub struct PipelineTriggerResult {
    pub received: Option<bool>,
    pub trace_id: Option<PipelineTriggerTraceId>,
    /// id of trigger data,
    /// type must be str since length of value beyonds the limitation of serialization of javascript json number
    pub internal_data_id: Option<TopicDataId>,
    /// actually, pipeline log is a topic as well.
    pub log_id: Option<TopicDataId>,
}

pub enum TopicDataColumnNames {
    Id,
    RawTopicData,
    AggregateAssist,
    Version,
    TenantId,
    InsertTime,
    UpdateTime,
}

impl TopicDataColumnNames {
    pub fn column_name(self) -> &'static str {
        match self {
            Self::Id => "id_",
            Self::RawTopicData => "data_",
            Self::AggregateAssist => "aggregate_assist_",
            Self::Version => "version_",
            Self::TenantId => "tenant_id_",
            Self::InsertTime => "insert_time_",
            Self::UpdateTime => "update_time_",
        }
    }
}
