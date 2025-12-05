use crate::{
    CompiledParameter, InMemoryParameter, InMemoryParameterCondition, PipelineExecutionVariables,
};
use std::sync::Arc;
use watchmen_model::StdR;
use watchmen_runtime_model_kernel::ArcNotInExpression;

pub struct CompiledNotInExpression {
    left: CompiledParameter,
    right: CompiledParameter,
}

impl CompiledNotInExpression {
    pub fn new(exp: Arc<ArcNotInExpression>) -> StdR<Self> {
        Ok(CompiledNotInExpression {
            left: CompiledParameter::new(exp.left.clone())?,
            right: CompiledParameter::new(exp.right.clone())?,
        })
    }
}

impl InMemoryParameterCondition for CompiledNotInExpression {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)?
            .is_not_in(self.right.value_from(variables)?)
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.left
            .value_from(variables)?
            .is_in(self.right.value_from(variables)?)
    }
}
