use crate::{
    ArcTopicDataValue, DataPath, DataVisitor, InMemoryParameter, PipelineExecutionVariables,
    PipelineKernelErrorCode,
};
use elf_base::{ErrorCode, StdR};
use elf_model::TenantId;
use elf_runtime_model_kernel::{ArcTopicFactorParameter, TopicSchemaProvider, TopicService};
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledTopicFactorParameter {
    path: DataPath,
}

impl CompiledTopicFactorParameter {
    pub fn new(parameter: &Arc<ArcTopicFactorParameter>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        let topic_schema = TopicService::schema()?.by_id(parameter.topic_id.as_ref(), tenant_id)?;
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
impl InMemoryParameter for CompiledTopicFactorParameter {
    fn value_from(&self, variables: &PipelineExecutionVariables) -> StdR<Arc<ArcTopicDataValue>> {
        variables.get_current_data()?.value_of(&self.path)
    }
}
