use crate::{
    CompiledParameter, InMemoryParameter, InMemoryParameterCondition, PipelineExecutionVariables,
};
use std::sync::Arc;
use watchmen_model::{StdR, TenantId};
use watchmen_runtime_model_kernel::ArcInExpression;

pub struct CompiledInExpression {
    left: CompiledParameter,
    right: CompiledParameter,
}

impl CompiledInExpression {
    pub fn new(exp: &Arc<ArcInExpression>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        Ok(CompiledInExpression {
            left: CompiledParameter::new(&exp.left, tenant_id)?,
            right: CompiledParameter::new(&exp.right, tenant_id)?,
        })
    }
}

impl InMemoryParameterCondition for CompiledInExpression {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)?
            .is_in(self.right.value_from(variables)?)
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)?
            .is_not_in(self.right.value_from(variables)?)
    }
}
