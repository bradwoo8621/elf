use crate::{
	BaseDataModel, ConnectedSpaceId, DashboardId, DerivedObjectiveId, LastVisit, Storable,
	TenantId, UserBasedTuple, UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

#[adapt_model(user_based, last_visit)]
pub struct Favorite {
    pub connected_space_ids: Option<Vec<ConnectedSpaceId>>,
    pub dashboard_ids: Option<Vec<DashboardId>>,
    pub derived_objective_ids: Option<Vec<DerivedObjectiveId>>,
}
