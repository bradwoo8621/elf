use crate::{
	Auditable, BaseDataModel, ConnectedSpaceId, LastVisit, Report, Storable, SubjectDataset,
	SubjectId, TenantId, UserBasedTuple, UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

/// extend a [reports] field from [Subject]
#[adapt_model(user_based, audit, last_visit)]
pub struct SubjectWithReports {
    pub subject_id: Option<SubjectId>,
    pub name: Option<String>,
    pub connect_id: Option<ConnectedSpaceId>,
    pub auto_refresh_interval: Option<i32>,
    pub dataset: Option<SubjectDataset>,
    pub reports: Option<Report>,
}
