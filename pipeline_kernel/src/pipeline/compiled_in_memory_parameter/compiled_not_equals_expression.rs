use crate::{
    CompiledParameter, InMemoryParameter, InMemoryParameterCondition, PipelineExecutionVariables,
};
use std::sync::Arc;
use watchmen_model::StdR;
use watchmen_runtime_model_kernel::ArcNotEqualsExpression;

pub struct CompiledNotEqualsExpression {
    left: CompiledParameter,
    right: CompiledParameter,
}

impl CompiledNotEqualsExpression {
    pub fn new(exp: Arc<ArcNotEqualsExpression>) -> StdR<Self> {
        Ok(CompiledNotEqualsExpression {
            left: CompiledParameter::new(exp.left.clone())?,
            right: CompiledParameter::new(exp.right.clone())?,
        })
    }
}

impl InMemoryParameterCondition for CompiledNotEqualsExpression {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        Ok(self
            .left
            .value_from(variables)?
            .is_not_same_as(self.right.value_from(variables)?))
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        Ok(self
            .left
            .value_from(variables)?
            .is_same_as(self.right.value_from(variables)?))
    }
}
