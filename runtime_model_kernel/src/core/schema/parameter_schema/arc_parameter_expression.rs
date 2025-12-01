use crate::{
    ArcEmptyExpression, ArcEqualsExpression, ArcInExpression, ArcLessThanExpression,
    ArcLessThanOrEqualsExpression, ArcMoreThanExpression, ArcMoreThanOrEqualsExpression,
    ArcNotEmptyExpression, ArcNotEqualsExpression, ArcNotInExpression,
};
use std::sync::Arc;
use watchmen_model::{ParameterExpression, StdR};

#[derive(Debug)]
pub enum ArcParameterExpression {
    Empty(ArcEmptyExpression),
    NotEmpty(ArcNotEmptyExpression),
    Equals(ArcEqualsExpression),
    NotEquals(ArcNotEqualsExpression),
    LessThan(ArcLessThanExpression),
    LessThanOrEquals(ArcLessThanOrEqualsExpression),
    MoreThan(ArcMoreThanExpression),
    MoreThanOrEquals(ArcMoreThanOrEqualsExpression),
    In(ArcInExpression),
    NotIn(ArcNotInExpression),
}

impl ArcParameterExpression {
    pub fn new(expression: ParameterExpression) -> StdR<Self> {
        let arc_expression = match expression {
            ParameterExpression::Empty(exp) => {
                ArcParameterExpression::Empty(ArcEmptyExpression::new(exp)?)
            }
            ParameterExpression::NotEmpty(exp) => {
                ArcParameterExpression::NotEmpty(ArcNotEmptyExpression::new(exp)?)
            }
            ParameterExpression::Equals(exp) => {
                ArcParameterExpression::Equals(ArcEqualsExpression::new(exp)?)
            }
            ParameterExpression::NotEquals(exp) => {
                ArcParameterExpression::NotEquals(ArcNotEqualsExpression::new(exp)?)
            }
            ParameterExpression::LessThan(exp) => {
                ArcParameterExpression::LessThan(ArcLessThanExpression::new(exp)?)
            }
            ParameterExpression::LessThanOrEquals(exp) => {
                ArcParameterExpression::LessThanOrEquals(ArcLessThanOrEqualsExpression::new(exp)?)
            }
            ParameterExpression::MoreThan(exp) => {
                ArcParameterExpression::MoreThan(ArcMoreThanExpression::new(exp)?)
            }
            ParameterExpression::MoreThanOrEquals(exp) => {
                ArcParameterExpression::MoreThanOrEquals(ArcMoreThanOrEqualsExpression::new(exp)?)
            }
            ParameterExpression::In(exp) => ArcParameterExpression::In(ArcInExpression::new(exp)?),
            ParameterExpression::NotIn(exp) => {
                ArcParameterExpression::NotIn(ArcNotInExpression::new(exp)?)
            }
        };

        Ok(arc_expression)
    }

    pub fn new_arc(expression: ParameterExpression) -> StdR<Arc<Self>> {
        Ok(Arc::new(Self::new(expression)?))
    }
}
