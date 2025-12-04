use crate::{
    CompiledEmptyExpression, CompiledEqualsExpression, CompiledInExpression,
    CompiledLessThanExpression, CompiledLessThanOrEqualsExpression, CompiledMoreThanExpression,
    CompiledMoreThanOrEqualsExpression, CompiledNotEmptyExpression, CompiledNotEqualsExpression,
    CompiledNotInExpression, InMemoryParameterCondition, PipelineExecutionVariables,
};
use std::ops::Deref;
use std::sync::Arc;
use watchmen_model::StdR;
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
    pub fn new(value: Arc<ArcParameterExpression>) -> Self {
        match value.deref() {
            ArcParameterExpression::Empty(v) => {
                CompiledParameterExpression::Empty(CompiledEmptyExpression::new(v.clone()))
            }
            ArcParameterExpression::NotEmpty(v) => {
                CompiledParameterExpression::NotEmpty(CompiledNotEmptyExpression::new(v.clone()))
            }
            ArcParameterExpression::Equals(v) => {
                CompiledParameterExpression::Equals(CompiledEqualsExpression::new(v.clone()))
            }
            ArcParameterExpression::NotEquals(v) => {
                CompiledParameterExpression::NotEquals(CompiledNotEqualsExpression::new(v.clone()))
            }
            ArcParameterExpression::LessThan(v) => {
                CompiledParameterExpression::LessThan(CompiledLessThanExpression::new(v.clone()))
            }
            ArcParameterExpression::LessThanOrEquals(v) => {
                CompiledParameterExpression::LessThanOrEquals(
                    CompiledLessThanOrEqualsExpression::new(v.clone()),
                )
            }
            ArcParameterExpression::MoreThan(v) => {
                CompiledParameterExpression::MoreThan(CompiledMoreThanExpression::new(v.clone()))
            }
            ArcParameterExpression::MoreThanOrEquals(v) => {
                CompiledParameterExpression::MoreThanOrEquals(
                    CompiledMoreThanOrEqualsExpression::new(v.clone()),
                )
            }
            ArcParameterExpression::In(v) => {
                CompiledParameterExpression::In(CompiledInExpression::new(v.clone()))
            }
            ArcParameterExpression::NotIn(v) => {
                CompiledParameterExpression::NotIn(CompiledNotInExpression::new(v.clone()))
            }
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
