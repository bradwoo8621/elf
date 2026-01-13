use crate::{CompiledParameterExpression, CompiledParameterJoint, InMemoryData};
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
    pub fn compile(value: &Arc<ArcParameterCondition>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        match value.deref() {
            ArcParameterCondition::Expression(v) => {
                CompiledParameterExpression::compile(v, tenant_id)
                    .map(|p| CompiledParameterCondition::Expression(p))
            }
            ArcParameterCondition::Joint(v) => CompiledParameterJoint::compile(v, tenant_id)
                .map(|p| CompiledParameterCondition::Joint(p)),
        }
    }
}

impl CompiledParameterCondition {
    pub fn is_true(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        match self {
            Self::Expression(v) => v.is_true(in_memory_data),
            Self::Joint(v) => v.is_true(in_memory_data),
        }
    }

    pub fn is_false(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        match self {
            Self::Expression(v) => v.is_false(in_memory_data),
            Self::Joint(v) => v.is_false(in_memory_data),
        }
    }
}
