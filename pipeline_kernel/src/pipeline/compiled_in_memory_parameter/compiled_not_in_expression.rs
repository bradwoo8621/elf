use crate::{
	CompiledParameter, InMemoryParameter, InMemoryParameterCondition, PipelineExecutionVariables,
};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcNotInExpression;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledNotInExpression {
    left: CompiledParameter,
    right: CompiledParameter,
}

impl CompiledNotInExpression {
    pub fn new(exp: &Arc<ArcNotInExpression>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        Ok(CompiledNotInExpression {
            left: CompiledParameter::new(&exp.left, tenant_id)?,
            right: CompiledParameter::new(&exp.right, tenant_id)?,
        })
    }
}

impl InMemoryParameterCondition for CompiledNotInExpression {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)?
            .is_not_in(self.right.value_from(variables)?.deref())
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)?
            .is_in(self.right.value_from(variables)?.deref())
    }
}
