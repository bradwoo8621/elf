use crate::{PipelineMetaProvider, PipelineSchema};
use std::sync::Arc;
use watchmen_base::StdR;
use watchmen_model::{PipelineId, TenantId, TopicId};

/// TODO pipeline meta service using tenant and it's meta datasource (or the global meta datasource)
///  to find out pipeline meta.
///  the tenant meta datasource is a new feature, which is defined on tenant
pub struct PipelineSchemaService;

impl PipelineMetaProvider for PipelineSchemaService {}

// TODO maybe find from cache
impl PipelineSchemaService {
    fn new() -> StdR<Arc<Self>> {
        Ok(Arc::new(Self {}))
    }

    pub fn by_pipeline_id(
        &self,
        pipeline_id: &PipelineId,
        tenant_id: &TenantId,
    ) -> StdR<Option<Arc<PipelineSchema>>> {
        let pipeline = Self::meta()?.by_pipeline_id(pipeline_id, tenant_id)?;
        if let Some(pipeline) = pipeline {
            let schema = PipelineSchema::new(pipeline)?;
            Ok(Some(Arc::new(schema)))
        } else {
            Ok(None)
        }
    }

    pub fn by_topic_id(
        &self,
        topic_id: &TopicId,
        tenant_id: &TenantId,
    ) -> StdR<Option<Vec<Arc<PipelineSchema>>>> {
        let pipelines = Self::meta()?.by_topic_id(topic_id, tenant_id)?;
        match pipelines {
            Some(pipelines) => {
                let mut schemas = vec![];
                for pipeline in pipelines {
                    schemas.push(Arc::new(PipelineSchema::new(pipeline)?));
                }
                Ok(Some(schemas))
            }
            _ => Ok(None),
        }
    }
}

pub trait PipelineSchemaProvider {
    fn schema() -> StdR<Arc<PipelineSchemaService>> {
        PipelineSchemaService::new()
    }
}
