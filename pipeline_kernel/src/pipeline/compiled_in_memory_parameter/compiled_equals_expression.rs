use crate::{
	CompiledParameter, InMemoryParameter, InMemoryParameterCondition, PipelineExecutionVariables,
};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcEqualsExpression;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledEqualsExpression {
    left: CompiledParameter,
    right: CompiledParameter,
}

impl CompiledEqualsExpression {
    pub fn new(exp: &Arc<ArcEqualsExpression>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        Ok(CompiledEqualsExpression {
            left: CompiledParameter::new(&exp.left, tenant_id)?,
            right: CompiledParameter::new(&exp.right, tenant_id)?,
        })
    }
}

impl InMemoryParameterCondition for CompiledEqualsExpression {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        Ok(self
            .left
            .value_from(variables)?
            .is_same_as(&self.right.value_from(variables)?.deref()))
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        Ok(self
            .left
            .value_from(variables)?
            .is_not_same_as(&self.right.value_from(variables)?.deref()))
    }
}
