use crate::{ArcTopicDataValue, InMemoryData};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcConstantParameter;
use std::sync::Arc;

pub struct CompiledConstantParameter;

impl CompiledConstantParameter {
    pub fn compile(
        _parameter: &Arc<ArcConstantParameter>,
        _tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        Ok(CompiledConstantParameter {})
    }
}

impl CompiledConstantParameter {
    pub fn value_from(&self, _in_memory_data: &mut InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        todo!("implement value_from for CompiledConstantParameter")
    }
}
