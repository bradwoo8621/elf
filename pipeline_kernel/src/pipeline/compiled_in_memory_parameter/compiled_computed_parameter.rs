use crate::{ArcTopicDataValue, InMemoryData};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcComputedParameter;
use std::sync::Arc;

pub struct CompiledComputedParameter;

impl CompiledComputedParameter {
    pub fn compile(
        _parameter: &Arc<ArcComputedParameter>,
        _tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        Ok(CompiledComputedParameter {})
    }
}

impl CompiledComputedParameter {
    pub fn value_from(&self, _in_memory_data: &mut InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        todo!("implement value_from for CompiledComputedParameter")
    }
}
