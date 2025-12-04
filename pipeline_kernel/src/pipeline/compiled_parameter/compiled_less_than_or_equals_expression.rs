use crate::{
    CompiledParameter, InMemoryParameter, InMemoryParameterCondition, PipelineExecutionVariables,
};
use std::sync::Arc;
use watchmen_model::StdR;
use watchmen_runtime_model_kernel::ArcLessThanOrEqualsExpression;

pub struct CompiledLessThanOrEqualsExpression {
    left: CompiledParameter,
    right: CompiledParameter,
}

impl CompiledLessThanOrEqualsExpression {
    pub fn new(exp: Arc<ArcLessThanOrEqualsExpression>) -> Self {
        CompiledLessThanOrEqualsExpression {
            left: CompiledParameter::new(exp.left.clone()),
            right: CompiledParameter::new(exp.right.clone()),
        }
    }
}

impl InMemoryParameterCondition for CompiledLessThanOrEqualsExpression {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)
            .is_less_than_or_equals(self.right.value_from(variables))
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)
            .is_more_than(self.right.value_from(variables))
    }
}
