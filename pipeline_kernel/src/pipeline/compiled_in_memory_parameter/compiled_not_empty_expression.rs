use crate::{
    CompiledParameter, InMemoryParameter, InMemoryParameterCondition, PipelineExecutionVariables,
};
use std::sync::Arc;
use watchmen_model::StdR;
use watchmen_runtime_model_kernel::ArcNotEmptyExpression;

pub struct CompiledNotEmptyExpression {
    left: CompiledParameter,
}

impl CompiledNotEmptyExpression {
    pub fn new(exp: Arc<ArcNotEmptyExpression>) -> StdR<Self> {
        Ok(CompiledNotEmptyExpression {
            left: CompiledParameter::new(exp.left.clone())?,
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
