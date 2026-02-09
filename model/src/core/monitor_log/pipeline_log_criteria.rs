use crate::{
    BaseDataModel, MonitorLogStatus, Pageable, PipelineId, PipelineTriggerTraceId, Storable,
    TenantId, TopicId,
};
use elf_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct PipelineMonitorLogCriteria {
    pub topic_id: Option<TopicId>,
    pub pipeline_id: Option<PipelineId>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub status: Option<MonitorLogStatus>,
    pub trace_id: Option<PipelineTriggerTraceId>,
    pub tenant_id: Option<TenantId>,
    /// [Pageable]
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
}

impl Pageable for PipelineMonitorLogCriteria {
    fn page_number(&self) -> u32 {
        if let Some(page_number) = self.page_number {
            page_number
        } else {
            1
        }
    }

    fn page_size(&self) -> u32 {
        if let Some(page_size) = self.page_size {
            page_size
        } else {
            20
        }
    }
}
