use crate::{
    CompiledParameterExpression, CompiledParameterJoint, InMemoryParameterCondition,
    PipelineExecutionVariables,
};
use std::ops::Deref;
use std::sync::Arc;
use watchmen_model::StdR;
use watchmen_runtime_model_kernel::ArcParameterCondition;

pub enum CompiledParameterCondition {
    Joint(CompiledParameterJoint),
    Expression(CompiledParameterExpression),
}

impl CompiledParameterCondition {
    pub fn new(value: Arc<ArcParameterCondition>) -> Self {
        match value.deref() {
            ArcParameterCondition::Expression(v) => {
                CompiledParameterCondition::Expression(CompiledParameterExpression::new(v.clone()))
            }
            ArcParameterCondition::Joint(v) => {
                CompiledParameterCondition::Joint(CompiledParameterJoint::new(v.clone()))
            }
        }
    }
}

impl InMemoryParameterCondition for CompiledParameterCondition {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        match self {
            CompiledParameterCondition::Expression(v) => v.is_true(variables),
            CompiledParameterCondition::Joint(v) => v.is_true(variables),
        }
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        match self {
            CompiledParameterCondition::Expression(v) => v.is_false(variables),
            CompiledParameterCondition::Joint(v) => v.is_false(variables),
        }
    }
}
