use crate::{
    CompiledParameter, InMemoryParameter, InMemoryParameterCondition, PipelineExecutionVariables,
};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcLessThanExpression;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledLessThanExpression {
    left: CompiledParameter,
    right: CompiledParameter,
}

impl CompiledLessThanExpression {
    pub fn new(exp: &Arc<ArcLessThanExpression>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        Ok(CompiledLessThanExpression {
            left: CompiledParameter::new(&exp.left, tenant_id)?,
            right: CompiledParameter::new(&exp.right, tenant_id)?,
        })
    }
}

impl InMemoryParameterCondition for CompiledLessThanExpression {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)?
            .is_less_than(self.right.value_from(variables)?.deref())
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)?
            .is_more_than_or_equals(self.right.value_from(variables)?.deref())
    }
}
