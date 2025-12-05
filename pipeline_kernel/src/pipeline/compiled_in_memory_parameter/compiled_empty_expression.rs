use crate::{
    CompiledParameter, InMemoryParameter, InMemoryParameterCondition, PipelineExecutionVariables,
};
use std::sync::Arc;
use watchmen_model::{StdR, TenantId};
use watchmen_runtime_model_kernel::ArcEmptyExpression;

pub struct CompiledEmptyExpression {
    left: CompiledParameter,
}

impl CompiledEmptyExpression {
    pub fn new(exp: &Arc<ArcEmptyExpression>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        Ok(CompiledEmptyExpression {
            left: CompiledParameter::new(&exp.left, tenant_id)?,
        })
    }
}

impl InMemoryParameterCondition for CompiledEmptyExpression {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        Ok(self.left.value_from(variables)?.is_empty())
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        Ok(self.left.value_from(variables)?.is_not_empty())
    }
}
