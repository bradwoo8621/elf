use crate::serde::option_naive_datetime;
use crate::{
    BaseDataModel, FactorId, PipelineActionId, PipelineId, PipelineStageId, PipelineUnitId,
    StdErrCode, StdErrorCode, StdR, Storable, TenantId, TopicId,
};
use chrono::NaiveDateTime;
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum PipelineIndexRefType {
    Direct,
    Computed,
}

pub type PipelineIndexId = String;

#[adapt_model(storable)]
pub struct PipelineIndex {
    pub pipeline_index_id: Option<PipelineIndexId>,
    pub pipeline_id: Option<PipelineId>,
    pub pipeline_name: Option<String>,
    pub stage_id: Option<PipelineStageId>,
    pub stage_name: Option<String>,
    pub unit_id: Option<PipelineUnitId>,
    pub unit_name: Option<String>,
    pub action_id: Option<PipelineActionId>,
    pub mapping_to_topic_id: Option<TopicId>,
    pub mapping_to_factor_id: Option<FactorId>,
    pub source_from_topic_id: Option<TopicId>,
    pub source_from_factor_id: Option<FactorId>,
    pub ref_type: Option<PipelineIndexRefType>,
    pub tenant_id: Option<TenantId>,
    #[serde(with = "option_naive_datetime")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(with = "option_naive_datetime")]
    pub last_modified_at: Option<NaiveDateTime>,
}
