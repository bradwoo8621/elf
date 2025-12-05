use crate::{
    CompiledEmptyExpression, CompiledEqualsExpression, CompiledInExpression,
    CompiledLessThanExpression, CompiledLessThanOrEqualsExpression, CompiledMoreThanExpression,
    CompiledMoreThanOrEqualsExpression, CompiledNotEmptyExpression, CompiledNotEqualsExpression,
    CompiledNotInExpression, InMemoryParameterCondition, PipelineExecutionVariables,
};
use std::ops::Deref;
use std::sync::Arc;
use watchmen_model::{StdR, TenantId};
use watchmen_runtime_model_kernel::ArcParameterExpression;

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
            CompiledParameterExpression::Empty(v) => v.is_true(variables),
            CompiledParameterExpression::NotEmpty(v) => v.is_true(variables),
            CompiledParameterExpression::Equals(v) => v.is_true(variables),
            CompiledParameterExpression::NotEquals(v) => v.is_true(variables),
            CompiledParameterExpression::LessThan(v) => v.is_true(variables),
            CompiledParameterExpression::LessThanOrEquals(v) => v.is_true(variables),
            CompiledParameterExpression::MoreThan(v) => v.is_true(variables),
            CompiledParameterExpression::MoreThanOrEquals(v) => v.is_true(variables),
            CompiledParameterExpression::In(v) => v.is_true(variables),
            CompiledParameterExpression::NotIn(v) => v.is_true(variables),
        }
    }

    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        match self {
            CompiledParameterExpression::Empty(v) => v.is_false(variables),
            CompiledParameterExpression::NotEmpty(v) => v.is_false(variables),
            CompiledParameterExpression::Equals(v) => v.is_false(variables),
            CompiledParameterExpression::NotEquals(v) => v.is_false(variables),
            CompiledParameterExpression::LessThan(v) => v.is_false(variables),
            CompiledParameterExpression::LessThanOrEquals(v) => v.is_false(variables),
            CompiledParameterExpression::MoreThan(v) => v.is_false(variables),
            CompiledParameterExpression::MoreThanOrEquals(v) => v.is_false(variables),
            CompiledParameterExpression::In(v) => v.is_false(variables),
            CompiledParameterExpression::NotIn(v) => v.is_false(variables),
        }
    }
}
