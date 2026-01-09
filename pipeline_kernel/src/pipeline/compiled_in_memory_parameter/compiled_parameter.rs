use crate::{
	ArcTopicDataValue, CompiledComputedParameter, CompiledConstantParameter,
	CompiledTopicFactorParameter, InMemoryParameter, PipelineExecutionVariables,
};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcParameter;
use std::ops::Deref;
use std::sync::Arc;

pub enum CompiledParameter {
    Topic(CompiledTopicFactorParameter),
    Constant(CompiledConstantParameter),
    Computed(CompiledComputedParameter),
}

impl CompiledParameter {
    pub fn new(value: &Arc<ArcParameter>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        match value.deref() {
            ArcParameter::Topic(v) => {
                CompiledTopicFactorParameter::new(v, tenant_id).map(|p| CompiledParameter::Topic(p))
            }
            ArcParameter::Constant(v) => {
                CompiledConstantParameter::new(v, tenant_id).map(|p| CompiledParameter::Constant(p))
            }
            ArcParameter::Computed(v) => {
                CompiledComputedParameter::new(v, tenant_id).map(|p| CompiledParameter::Computed(p))
            }
        }
    }
}

impl InMemoryParameter for CompiledParameter {
    fn value_from(&self, variables: &PipelineExecutionVariables) -> StdR<Arc<ArcTopicDataValue>> {
        match self {
            Self::Topic(v) => v.value_from(variables),
            Self::Constant(v) => v.value_from(variables),
            Self::Computed(v) => v.value_from(variables),
        }
    }
}
