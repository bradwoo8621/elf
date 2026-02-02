use crate::{ArcTopicDataValue, DataPath, InMemoryData, PipelineKernelErrorCode};
use elf_base::{ErrorCode, StdR, StringUtils};
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
        let value = &parameter.value;
        if value.is_empty() {
            PipelineKernelErrorCode::ConstantParameterIsEmpty
                .msg("Value of constant parameter cannot be empty.")
        } else if value.is_blank() {
            PipelineKernelErrorCode::ConstantParameterIsBlank
                .msg("Value of constant parameter cannot be blank.")
        } else {
            Ok(CompiledConstantParameter {
                path: DataPath::from_str(parameter.value.as_str())?,
            })
        }
    }
}

impl CompiledConstantParameter {
    pub fn value_from(&self, in_memory_data: &mut InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        in_memory_data.all_allowed().value_of(&self.path)
    }
}
