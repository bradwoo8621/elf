use crate::{CompiledParameterJoint, InMemoryParameterCondition, PipelineExecutionVariables};
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
    pub fn new(
        conditional: &Option<Arc<ArcParameterJoint>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        Ok(if let Some(conditional) = &conditional {
            CompiledConditional {
                inner: Some(CompiledParameterJoint::new(conditional, tenant_id)?),
            }
        } else {
            CompiledConditional { inner: None }
        })
    }

    pub fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.inner
            .as_ref()
            .map(|inner| inner.is_true(variables))
            // returns true when no condition
            .unwrap_or(Ok(true))
    }

    pub fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.inner
            .as_ref()
            .map(|inner| inner.is_false(variables))
            // returns false when no condition
            .unwrap_or(Ok(false))
    }
}
