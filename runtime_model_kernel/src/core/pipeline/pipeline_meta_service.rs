use elf_base::StdR;
use elf_model::{Pipeline, PipelineId, TenantId, TopicId};
use std::sync::Arc;

/// TODO pipeline meta service using tenant and it's meta datasource (or the global meta datasource)
///  to find out pipeline meta.
///  the tenant meta datasource is a new feature, which is defined on tenant
pub struct PipelineMetaService;

impl PipelineMetaService {
    fn new() -> StdR<Arc<Self>> {
        // TODO maybe find from cache
        Ok(Arc::new(Self {}))
    }

    pub fn by_pipeline_id(
        &self,
        _pipeline_id: &PipelineId,
        _tenant_id: &TenantId,
    ) -> StdR<Option<Pipeline>> {
        todo!("implement find_by_id for PipelineMetaService")
    }

    pub fn by_topic_id(
        &self,
        _topic_id: &TopicId,
        _tenant_id: &TenantId,
    ) -> StdR<Option<Vec<Pipeline>>> {
        todo!("implement find_pipeline_by_topic for PipelineMetaService")
    }
}

pub trait PipelineMetaProvider {
    fn meta() -> StdR<Arc<PipelineMetaService>> {
        PipelineMetaService::new()
    }
}
