use crate::{BaseDataModel, Parameter, Storable};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum ParameterExpressionOperator {
    Empty,
    NotEmpty,
    Equals,
    NotEquals,
    Less,
    LessEquals,
    More,
    MoreEquals,
    In,
    NotIn,
}

#[adapt_model(storable)]
pub enum ParameterExpression {
    Empty(Option<Parameter>),
    NotEmpty(Option<Parameter>),
    Equal(Option<Parameter>, Option<Parameter>),
    NotEqual(Option<Parameter>, Option<Parameter>),
    LessThan(Option<Parameter>, Option<Parameter>),
    LessThanOrEqual(Option<Parameter>, Option<Parameter>),
    MoreThan(Option<Parameter>, Option<Parameter>),
    MoreThanOrEqual(Option<Parameter>, Option<Parameter>),
    In(Option<Vec<Parameter>>),
    NotIn(Option<Vec<Parameter>>),
}

impl ParameterExpression {
    pub fn operator(&self) -> ParameterExpressionOperator {
        match self {
            ParameterExpression::Empty(_) => ParameterExpressionOperator::Empty,
            ParameterExpression::NotEmpty(_) => ParameterExpressionOperator::NotEmpty,
            ParameterExpression::Equal(_, _) => ParameterExpressionOperator::Equals,
            ParameterExpression::NotEqual(_, _) => ParameterExpressionOperator::NotEquals,
            ParameterExpression::LessThan(_, _) => ParameterExpressionOperator::Less,
            ParameterExpression::LessThanOrEqual(_, _) => ParameterExpressionOperator::LessEquals,
            ParameterExpression::MoreThan(_, _) => ParameterExpressionOperator::More,
            ParameterExpression::MoreThanOrEqual(_, _) => ParameterExpressionOperator::MoreEquals,
            ParameterExpression::In(_) => ParameterExpressionOperator::In,
            ParameterExpression::NotIn(_) => ParameterExpressionOperator::NotIn,
        }
    }
}
