use crate::{
	Auditable, BaseDataModel, LastVisit, SpaceId, Storable, TenantId, UserBasedTuple, UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

pub type ConnectedSpaceId = String;

#[adapt_model(user_based, audit, last_visit)]
pub struct ConnectedSpace {
    pub connect_id: Option<ConnectedSpaceId>,
    pub space_id: Option<SpaceId>,
    pub name: Option<String>,
    pub is_template: Option<bool>,
}
