use crate::{ArcTopicDataValue, DataPath, InMemoryData};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcConstantParameter;
use std::sync::Arc;

pub struct CompiledConstantParameter {
    path: DataPath,
}

impl CompiledConstantParameter {
    pub fn compile(
        parameter: &Arc<ArcConstantParameter>,
        _tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        Ok(CompiledConstantParameter {
            path: DataPath::from_str(parameter.value.as_str())?,
        })
    }
}

impl CompiledConstantParameter {
    pub fn value_from(&self, in_memory_data: &mut InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        in_memory_data.all_allowed().value_of(&self.path)
    }
}
