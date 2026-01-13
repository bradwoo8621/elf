use crate::{CompiledParameterJoint, InMemoryData};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcParameterJoint;
use std::sync::Arc;

/// in-memory check
pub struct CompiledConditional {
    /// is some only when should is true, otherwise is none
    inner: Option<CompiledParameterJoint>,
}

impl CompiledConditional {
    pub fn compile(
        conditional: &Option<Arc<ArcParameterJoint>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        Ok(if let Some(conditional) = &conditional {
            CompiledConditional {
                inner: Some(CompiledParameterJoint::compile(conditional, tenant_id)?),
            }
        } else {
            CompiledConditional { inner: None }
        })
    }

    pub fn is_true(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        self.inner
            .as_ref()
            .map(|inner| inner.is_true(in_memory_data))
            // returns true when no condition
            .unwrap_or(Ok(true))
    }

    pub fn is_false(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        self.inner
            .as_ref()
            .map(|inner| inner.is_false(in_memory_data))
            // returns false when no condition
            .unwrap_or(Ok(false))
    }
}
