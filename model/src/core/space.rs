use crate::{
    Auditable, BaseDataModel, OptimisticLock, ParameterJoint, Storable, TenantBasedTuple, TenantId,
    TopicId, Tuple, UserGroupId, UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct SpaceFilter {
    pub topic_id: Option<TopicId>,
    pub enabled: Option<bool>,
    pub joint: Option<ParameterJoint>,
}

pub type SpaceId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct Space {
    pub space_id: Option<SpaceId>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub topic_ids: Option<Vec<TopicId>>,
    pub group_ids: Option<Vec<UserGroupId>>,
    pub filters: Option<Vec<SpaceFilter>>,
}
