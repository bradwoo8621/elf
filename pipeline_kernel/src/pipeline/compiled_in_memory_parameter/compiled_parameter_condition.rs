use crate::{
    CompiledParameterExpression, CompiledParameterJoint, InMemoryParameterCondition,
    PipelineExecutionVariables,
};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcParameterCondition;
use std::ops::Deref;
use std::sync::Arc;

pub enum CompiledParameterCondition {
    Joint(CompiledParameterJoint),
    Expression(CompiledParameterExpression),
}

impl CompiledParameterCondition {
    pub fn new(value: &Arc<ArcParameterCondition>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        match value.deref() {
            ArcParameterCondition::Expression(v) => CompiledParameterExpression::new(v, tenant_id)
                .map(|p| CompiledParameterCondition::Expression(p)),
            ArcParameterCondition::Joint(v) => CompiledParameterJoint::new(v, tenant_id)
                .map(|p| CompiledParameterCondition::Joint(p)),
        }
    }
}

impl InMemoryParameterCondition for CompiledParameterCondition {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        match self {
            Self::Expression(v) => v.is_true(variables),
            Self::Joint(v) => v.is_true(variables),
        }
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        match self {
            Self::Expression(v) => v.is_false(variables),
            Self::Joint(v) => v.is_false(variables),
        }
    }
}
