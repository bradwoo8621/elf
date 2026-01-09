use crate::{ArcTopicDataValue, InMemoryParameter, PipelineExecutionVariables};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcComputedParameter;
use std::sync::Arc;

pub struct CompiledComputedParameter;

impl CompiledComputedParameter {
    pub fn new(_parameter: &Arc<ArcComputedParameter>, _tenant_id: &Arc<TenantId>) -> StdR<Self> {
        Ok(CompiledComputedParameter {})
    }
}

impl InMemoryParameter for CompiledComputedParameter {
    fn value_from(&self, _variables: &PipelineExecutionVariables) -> StdR<Arc<ArcTopicDataValue>> {
        todo!("implement value_from for CompiledComputedParameter")
    }
}
