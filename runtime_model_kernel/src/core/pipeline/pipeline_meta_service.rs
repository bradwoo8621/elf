use std::sync::Arc;
use watchmen_model::{Pipeline, PipelineId, PipelineTriggerType, StdR, TenantId, TopicId};

/// TODO pipeline meta service using tenant and it's meta datasource (or the global meta datasource)
///  to find out pipeline meta.
///  the tenant meta datasource is a new feature, which is defined on tenant
pub struct PipelineMetaService {
    tenant_id: TenantId,
}

impl PipelineMetaService {
    pub fn with(tenant_id: &TenantId) -> StdR<Arc<Self>> {
        // TODO maybe find from cache
        Ok(Arc::new(Self {
            tenant_id: tenant_id.clone(),
        }))
    }

    pub fn find_by_id(&self, _pipeline_id: &PipelineId) -> StdR<Option<Pipeline>> {
        todo!("implement find_pipeline_by_id for PipelineMetaService")
    }

    pub fn find_by_topic_and_pipeline_type(
        &self,
        _topic_id: &TopicId,
        r#_type: &PipelineTriggerType,
    ) -> StdR<Vec<Pipeline>> {
        todo!("implement find_pipeline_by_topic for PipelineMetaService")
    }
}
