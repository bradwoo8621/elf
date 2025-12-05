use crate::{
    CompiledParameter, InMemoryParameter, InMemoryParameterCondition, PipelineExecutionVariables,
};
use std::sync::Arc;
use watchmen_model::StdR;
use watchmen_runtime_model_kernel::ArcLessThanExpression;

pub struct CompiledLessThanExpression {
    left: CompiledParameter,
    right: CompiledParameter,
}

impl CompiledLessThanExpression {
    pub fn new(exp: Arc<ArcLessThanExpression>) -> StdR<Self> {
        Ok(CompiledLessThanExpression {
            left: CompiledParameter::new(exp.left.clone())?,
            right: CompiledParameter::new(exp.right.clone())?,
        })
    }
}

impl InMemoryParameterCondition for CompiledLessThanExpression {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)?
            .is_less_than(self.right.value_from(variables)?)
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)?
            .is_more_than_or_equals(self.right.value_from(variables)?)
    }
}
