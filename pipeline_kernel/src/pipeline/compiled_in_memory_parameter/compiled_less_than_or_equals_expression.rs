use crate::{
	CompiledParameter, InMemoryParameter, InMemoryParameterCondition, PipelineExecutionVariables,
};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcLessThanOrEqualsExpression;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledLessThanOrEqualsExpression {
    left: CompiledParameter,
    right: CompiledParameter,
}

impl CompiledLessThanOrEqualsExpression {
    pub fn new(exp: &Arc<ArcLessThanOrEqualsExpression>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        Ok(CompiledLessThanOrEqualsExpression {
            left: CompiledParameter::new(&exp.left, tenant_id)?,
            right: CompiledParameter::new(&exp.right, tenant_id)?,
        })
    }
}

impl InMemoryParameterCondition for CompiledLessThanOrEqualsExpression {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)?
            .is_less_than_or_equals(self.right.value_from(variables)?.deref())
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)?
            .is_more_than(self.right.value_from(variables)?.deref())
    }
}
