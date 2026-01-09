use crate::{
	CompiledParameter, InMemoryParameter, InMemoryParameterCondition, PipelineExecutionVariables,
};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcMoreThanExpression;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledMoreThanExpression {
    left: CompiledParameter,
    right: CompiledParameter,
}

impl CompiledMoreThanExpression {
    pub fn new(exp: &Arc<ArcMoreThanExpression>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        Ok(CompiledMoreThanExpression {
            left: CompiledParameter::new(&exp.left, tenant_id)?,
            right: CompiledParameter::new(&exp.right, tenant_id)?,
        })
    }
}

impl InMemoryParameterCondition for CompiledMoreThanExpression {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)?
            .is_more_than(self.right.value_from(variables)?.deref())
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)?
            .is_less_than_or_equals(self.right.value_from(variables)?.deref())
    }
}
