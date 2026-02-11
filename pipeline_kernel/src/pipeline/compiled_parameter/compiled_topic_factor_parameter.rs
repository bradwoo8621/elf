use crate::{ArcTopicDataValue, DataPath, InMemoryData, PipelineKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcTopicFactorParameter, TopicSchema, TopicSchemaProvider, TopicService,
};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledTopicFactorParameter {
    path: DataPath,
}

impl CompiledTopicFactorParameter {
    pub fn compile(
        parameter: &Arc<ArcTopicFactorParameter>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        let topic_id = parameter.topic_id.deref();
        let topic_schema = if let Some(topic_schema) = topic_schemas.get(topic_id) {
            topic_schema.clone()
        } else {
            let topic_schema = TopicService::schema()?.by_id(topic_id, tenant_id)?;
            topic_schemas.insert(parameter.topic_id.clone(), topic_schema.clone());
            topic_schema
        };
        let path = match topic_schema.factor_by_id(parameter.factor_id.as_ref()) {
            None => {
                return PipelineKernelErrorCode::FactorNotFound.msg(format!(
                    "Factor[{}] not found in topic[{}].",
                    &parameter.factor_id, &parameter.topic_id
                ));
            }
            Some(factor) => DataPath::from_factor(factor, topic_schema.deref())?,
        };

        Ok(CompiledTopicFactorParameter { path })
    }
}

/// topic factor parameter always retrieve data from current trigger data
impl CompiledTopicFactorParameter {
    pub fn value_from(&self, in_memory_data: &mut InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        in_memory_data.current_only().value_of(&self.path)
    }
}
