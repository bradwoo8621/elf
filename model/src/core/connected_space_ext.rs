use crate::{
	Auditable, BaseDataModel, ConnectedSpaceId, LastVisit, SpaceId, Storable, SubjectWithReports,
	TenantId, UserBasedTuple, UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

/// extend a [subjects] field from [ConnectedSpace]
#[adapt_model(user_based, audit, last_visit)]
pub struct ConnectedSpaceWithSubjects {
    pub connect_id: Option<ConnectedSpaceId>,
    pub space_id: Option<SpaceId>,
    pub name: Option<String>,
    pub is_template: Option<bool>,
    pub subjects: Option<Vec<SubjectWithReports>>,
}
