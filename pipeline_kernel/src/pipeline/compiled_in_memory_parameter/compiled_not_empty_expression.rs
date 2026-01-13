use crate::{
    CompiledParameter, InMemoryParameter, InMemoryParameterCondition, PipelineExecutionVariables,
};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcNotEmptyExpression;
use std::sync::Arc;

pub struct CompiledNotEmptyExpression {
    left: CompiledParameter,
}

impl CompiledNotEmptyExpression {
    pub fn new(exp: &Arc<ArcNotEmptyExpression>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        Ok(CompiledNotEmptyExpression {
            left: CompiledParameter::new(&exp.left, tenant_id)?,
        })
    }
}

impl InMemoryParameterCondition for CompiledNotEmptyExpression {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        Ok(self.left.value_from(variables)?.is_not_empty())
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        Ok(self.left.value_from(variables)?.is_empty())
    }
}
