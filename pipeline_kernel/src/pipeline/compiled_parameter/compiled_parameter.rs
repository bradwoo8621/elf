use crate::{
    ArcTopicDataValue, CompiledComputedParameter, CompiledConstantParameter,
    CompiledTopicFactorParameter, InMemoryData,
};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{ArcParameter, TopicSchema};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub enum CompiledParameter {
    Topic(CompiledTopicFactorParameter),
    Constant(CompiledConstantParameter),
    Computed(CompiledComputedParameter),
}

impl CompiledParameter {
    pub fn compile(
        value: &Arc<ArcParameter>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        match value.deref() {
            ArcParameter::Topic(v) => {
                CompiledTopicFactorParameter::compile(v, topic_schemas, tenant_id)
                    .map(|p| CompiledParameter::Topic(p))
            }
            ArcParameter::Constant(v) => {
                CompiledConstantParameter::compile(v, topic_schemas, tenant_id)
                    .map(|p| CompiledParameter::Constant(p))
            }
            ArcParameter::Computed(v) => {
                CompiledComputedParameter::compile(v, topic_schemas, tenant_id)
                    .map(|p| CompiledParameter::Computed(p))
            }
        }
    }
}

impl CompiledParameter {
    pub fn value_from(&self, in_memory_data: &mut InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        match self {
            Self::Topic(v) => v.value_from(in_memory_data),
            Self::Constant(v) => v.value_from(in_memory_data),
            Self::Computed(v) => v.value_from(in_memory_data),
        }
    }
}
