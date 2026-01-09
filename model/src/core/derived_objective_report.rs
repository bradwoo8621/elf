use crate::{
	Auditable, BaseDataModel, LastVisit, ObjectiveReport, ObjectiveReportId, Storable, TenantId,
	UserBasedTuple, UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

pub type DerivedObjectiveReportId = String;

#[adapt_model(user_based, audit, last_visit)]
pub struct DerivedObjectiveReport {
    pub derived_objective_report_id: Option<DerivedObjectiveReportId>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub objective_report_id: Option<ObjectiveReportId>,
    pub definition: Option<ObjectiveReport>,
}
