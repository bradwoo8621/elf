use crate::{BaseDataModel, FactorId, FactorType, Storable, TenantId, TopicId};
use chrono::NaiveDateTime;
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

pub type FactorIndexId = String;

#[adapt_model(storable)]
pub struct FactorIndex {
    pub factor_index_id: Option<FactorIndexId>,
    pub factor_id: Option<FactorId>,
    pub factor_type: Option<FactorType>,
    pub factor_name: Option<String>,
    pub factor_label: Option<String>,
    pub factor_description: Option<String>,
    pub topic_id: Option<TopicId>,
    pub topic_name: Option<String>,
    pub tenant_id: Option<TenantId>,
    #[serde(with = "option_naive_datetime")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(with = "option_naive_datetime")]
    pub last_modified_at: Option<NaiveDateTime>,
}
