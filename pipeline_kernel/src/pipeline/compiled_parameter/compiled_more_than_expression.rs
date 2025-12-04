use crate::{
    CompiledParameter, InMemoryParameter, InMemoryParameterCondition, PipelineExecutionVariables,
};
use std::sync::Arc;
use watchmen_model::StdR;
use watchmen_runtime_model_kernel::ArcMoreThanExpression;

pub struct CompiledMoreThanExpression {
    left: CompiledParameter,
    right: CompiledParameter,
}

impl CompiledMoreThanExpression {
    pub fn new(exp: Arc<ArcMoreThanExpression>) -> Self {
        CompiledMoreThanExpression {
            left: CompiledParameter::new(exp.left.clone()),
            right: CompiledParameter::new(exp.right.clone()),
        }
    }
}

impl InMemoryParameterCondition for CompiledMoreThanExpression {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)
            .is_more_than(self.right.value_from(variables))
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)
            .is_less_than_or_equals(self.right.value_from(variables))
    }
}
