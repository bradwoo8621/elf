use crate::{
	Auditable, BaseDataModel, GraphicRect, LastVisit, ReportFunnel, ReportId, Storable, TenantId,
	UserBasedTuple, UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct DashboardReport {
    pub report_id: Option<ReportId>,
    pub funnels: Option<Vec<ReportFunnel>>,
    pub rect: Option<GraphicRect>,
}

#[adapt_model(storable)]
pub struct DashboardParagraph {
    pub content: Option<String>,
    pub rect: Option<GraphicRect>,
}

pub type DashboardId = String;

#[adapt_model(user_based, audit, last_visit)]
pub struct Dashboard {
    pub dashboard_id: Option<DashboardId>,
    pub name: Option<String>,
    pub reports: Option<Vec<DashboardReport>>,
    pub paragraphs: Option<Vec<DashboardParagraph>>,
    pub auto_refresh_interval: Option<i32>,
}
