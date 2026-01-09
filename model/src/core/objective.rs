use crate::{
	Auditable, BaseDataModel, ObjectiveFactor, ObjectiveTarget, ObjectiveTimeframe,
	ObjectiveVariable, OptimisticLock, Storable, TenantBasedTuple, TenantId, Tuple, UserGroupId,
	UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

pub type ObjectiveId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct Objective {
    pub objective_id: Option<ObjectiveId>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub time_frame: Option<ObjectiveTimeframe>,
    pub targets: Option<Vec<ObjectiveTarget>>,
    pub variables: Option<Vec<ObjectiveVariable>>,
    pub factors: Option<Vec<ObjectiveFactor>>,
    pub group_ids: Option<Vec<UserGroupId>>,
}
