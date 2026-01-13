use crate::{
    CompiledEmptyExpression, CompiledEqualsExpression, CompiledInExpression,
    CompiledLessThanExpression, CompiledLessThanOrEqualsExpression, CompiledMoreThanExpression,
    CompiledMoreThanOrEqualsExpression, CompiledNotEmptyExpression, CompiledNotEqualsExpression,
    CompiledNotInExpression, InMemoryData,
};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcParameterExpression;
use std::ops::Deref;
use std::sync::Arc;

pub enum CompiledParameterExpression {
    Empty(CompiledEmptyExpression),
    NotEmpty(CompiledNotEmptyExpression),
    Equals(CompiledEqualsExpression),
    NotEquals(CompiledNotEqualsExpression),
    LessThan(CompiledLessThanExpression),
    LessThanOrEquals(CompiledLessThanOrEqualsExpression),
    MoreThan(CompiledMoreThanExpression),
    MoreThanOrEquals(CompiledMoreThanOrEqualsExpression),
    In(CompiledInExpression),
    NotIn(CompiledNotInExpression),
}

impl CompiledParameterExpression {
    pub fn compile(value: &Arc<ArcParameterExpression>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        match value.deref() {
            ArcParameterExpression::Empty(v) => CompiledEmptyExpression::compile(v, tenant_id)
                .map(|p| CompiledParameterExpression::Empty(p)),
            ArcParameterExpression::NotEmpty(v) => {
                CompiledNotEmptyExpression::compile(v, tenant_id)
                    .map(|p| CompiledParameterExpression::NotEmpty(p))
            }
            ArcParameterExpression::Equals(v) => CompiledEqualsExpression::compile(v, tenant_id)
                .map(|p| CompiledParameterExpression::Equals(p)),
            ArcParameterExpression::NotEquals(v) => {
                CompiledNotEqualsExpression::compile(v, tenant_id)
                    .map(|p| CompiledParameterExpression::NotEquals(p))
            }
            ArcParameterExpression::LessThan(v) => {
                CompiledLessThanExpression::compile(v, tenant_id)
                    .map(|p| CompiledParameterExpression::LessThan(p))
            }
            ArcParameterExpression::LessThanOrEquals(v) => {
                CompiledLessThanOrEqualsExpression::compile(v, tenant_id)
                    .map(|p| CompiledParameterExpression::LessThanOrEquals(p))
            }
            ArcParameterExpression::MoreThan(v) => {
                CompiledMoreThanExpression::compile(v, tenant_id)
                    .map(|p| CompiledParameterExpression::MoreThan(p))
            }
            ArcParameterExpression::MoreThanOrEquals(v) => {
                CompiledMoreThanOrEqualsExpression::compile(v, tenant_id)
                    .map(|p| CompiledParameterExpression::MoreThanOrEquals(p))
            }
            ArcParameterExpression::In(v) => CompiledInExpression::compile(v, tenant_id)
                .map(|p| CompiledParameterExpression::In(p)),
            ArcParameterExpression::NotIn(v) => CompiledNotInExpression::compile(v, tenant_id)
                .map(|p| CompiledParameterExpression::NotIn(p)),
        }
    }
}

impl CompiledParameterExpression {
    pub fn is_true(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        match self {
            Self::Empty(v) => v.is_true(in_memory_data),
            Self::NotEmpty(v) => v.is_true(in_memory_data),
            Self::Equals(v) => v.is_true(in_memory_data),
            Self::NotEquals(v) => v.is_true(in_memory_data),
            Self::LessThan(v) => v.is_true(in_memory_data),
            Self::LessThanOrEquals(v) => v.is_true(in_memory_data),
            Self::MoreThan(v) => v.is_true(in_memory_data),
            Self::MoreThanOrEquals(v) => v.is_true(in_memory_data),
            Self::In(v) => v.is_true(in_memory_data),
            Self::NotIn(v) => v.is_true(in_memory_data),
        }
    }

    pub fn is_false(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        match self {
            Self::Empty(v) => v.is_false(in_memory_data),
            Self::NotEmpty(v) => v.is_false(in_memory_data),
            Self::Equals(v) => v.is_false(in_memory_data),
            Self::NotEquals(v) => v.is_false(in_memory_data),
            Self::LessThan(v) => v.is_false(in_memory_data),
            Self::LessThanOrEquals(v) => v.is_false(in_memory_data),
            Self::MoreThan(v) => v.is_false(in_memory_data),
            Self::MoreThanOrEquals(v) => v.is_false(in_memory_data),
            Self::In(v) => v.is_false(in_memory_data),
            Self::NotIn(v) => v.is_false(in_memory_data),
        }
    }
}
