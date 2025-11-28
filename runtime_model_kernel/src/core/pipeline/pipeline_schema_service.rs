use crate::{PipelineMetaService, PipelineSchema};
use std::sync::Arc;
use watchmen_model::{PipelineId, PipelineTriggerType, StdR, TenantId, TopicId};

/// TODO pipeline meta service using tenant and it's meta datasource (or the global meta datasource)
///  to find out pipeline meta.
///  the tenant meta datasource is a new feature, which is defined on tenant
pub struct PipelineSchemaService {
    tenant_id: TenantId,
    meta: Arc<PipelineMetaService>,
}

// TODO maybe find from cache
impl PipelineSchemaService {
    pub fn with(tenant_id: &TenantId) -> StdR<Arc<Self>> {
        Ok(Arc::new(Self {
            tenant_id: tenant_id.clone(),
            meta: PipelineMetaService::with(tenant_id)?,
        }))
    }

    pub fn find_by_id(&self, pipeline_id: &PipelineId) -> StdR<Option<Arc<PipelineSchema>>> {
        let pipeline = self.meta.find_by_id(pipeline_id)?;
        if let Some(pipeline) = pipeline {
            let schema = PipelineSchema::new(pipeline)?;
            Ok(Some(Arc::new(schema)))
        } else {
            Ok(None)
        }
    }

    pub fn find_by_topic_and_pipeline_type(
        &self,
        topic_id: &TopicId,
        r#type: &PipelineTriggerType,
    ) -> StdR<Vec<Arc<PipelineSchema>>> {
        let pipelines = self
            .meta
            .find_by_topic_and_pipeline_type(topic_id, r#type)?;

        let mut schemas = vec![];
        for pipeline in pipelines {
            schemas.push(Arc::new(PipelineSchema::new(pipeline)?));
        }
        Ok(schemas)
    }
}
