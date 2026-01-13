use crate::{
    CompiledEmptyExpression, CompiledEqualsExpression, CompiledInExpression,
    CompiledLessThanExpression, CompiledLessThanOrEqualsExpression, CompiledMoreThanExpression,
    CompiledMoreThanOrEqualsExpression, CompiledNotEmptyExpression, CompiledNotEqualsExpression,
    CompiledNotInExpression, InMemoryParameterCondition, PipelineExecutionVariables,
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
    pub fn new(value: &Arc<ArcParameterExpression>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        match value.deref() {
            ArcParameterExpression::Empty(v) => CompiledEmptyExpression::new(v, tenant_id)
                .map(|p| CompiledParameterExpression::Empty(p)),
            ArcParameterExpression::NotEmpty(v) => CompiledNotEmptyExpression::new(v, tenant_id)
                .map(|p| CompiledParameterExpression::NotEmpty(p)),
            ArcParameterExpression::Equals(v) => CompiledEqualsExpression::new(v, tenant_id)
                .map(|p| CompiledParameterExpression::Equals(p)),
            ArcParameterExpression::NotEquals(v) => CompiledNotEqualsExpression::new(v, tenant_id)
                .map(|p| CompiledParameterExpression::NotEquals(p)),
            ArcParameterExpression::LessThan(v) => CompiledLessThanExpression::new(v, tenant_id)
                .map(|p| CompiledParameterExpression::LessThan(p)),
            ArcParameterExpression::LessThanOrEquals(v) => {
                CompiledLessThanOrEqualsExpression::new(v, tenant_id)
                    .map(|p| CompiledParameterExpression::LessThanOrEquals(p))
            }
            ArcParameterExpression::MoreThan(v) => CompiledMoreThanExpression::new(v, tenant_id)
                .map(|p| CompiledParameterExpression::MoreThan(p)),
            ArcParameterExpression::MoreThanOrEquals(v) => {
                CompiledMoreThanOrEqualsExpression::new(v, tenant_id)
                    .map(|p| CompiledParameterExpression::MoreThanOrEquals(p))
            }
            ArcParameterExpression::In(v) => {
                CompiledInExpression::new(v, tenant_id).map(|p| CompiledParameterExpression::In(p))
            }
            ArcParameterExpression::NotIn(v) => CompiledNotInExpression::new(v, tenant_id)
                .map(|p| CompiledParameterExpression::NotIn(p)),
        }
    }
}

impl InMemoryParameterCondition for CompiledParameterExpression {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        match self {
            Self::Empty(v) => v.is_true(variables),
            Self::NotEmpty(v) => v.is_true(variables),
            Self::Equals(v) => v.is_true(variables),
            Self::NotEquals(v) => v.is_true(variables),
            Self::LessThan(v) => v.is_true(variables),
            Self::LessThanOrEquals(v) => v.is_true(variables),
            Self::MoreThan(v) => v.is_true(variables),
            Self::MoreThanOrEquals(v) => v.is_true(variables),
            Self::In(v) => v.is_true(variables),
            Self::NotIn(v) => v.is_true(variables),
        }
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        match self {
            Self::Empty(v) => v.is_false(variables),
            Self::NotEmpty(v) => v.is_false(variables),
            Self::Equals(v) => v.is_false(variables),
            Self::NotEquals(v) => v.is_false(variables),
            Self::LessThan(v) => v.is_false(variables),
            Self::LessThanOrEquals(v) => v.is_false(variables),
            Self::MoreThan(v) => v.is_false(variables),
            Self::MoreThanOrEquals(v) => v.is_false(variables),
            Self::In(v) => v.is_false(variables),
            Self::NotIn(v) => v.is_false(variables),
        }
    }
}
