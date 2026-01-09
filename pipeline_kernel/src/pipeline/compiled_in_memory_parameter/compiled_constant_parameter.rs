use crate::{ArcTopicDataValue, InMemoryParameter, PipelineExecutionVariables};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcConstantParameter;
use std::sync::Arc;

pub struct CompiledConstantParameter;

impl CompiledConstantParameter {
    pub fn new(_parameter: &Arc<ArcConstantParameter>, _tenant_id: &Arc<TenantId>) -> StdR<Self> {
        Ok(CompiledConstantParameter {})
    }
}

impl InMemoryParameter for CompiledConstantParameter {
    fn value_from(&self, _variables: &PipelineExecutionVariables) -> StdR<Arc<ArcTopicDataValue>> {
        todo!("implement value_from for CompiledConstantParameter")
    }
}
