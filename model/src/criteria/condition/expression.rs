use crate::{serde_for_enum, BaseDataModel, Parameter};
use watchmen_model_marco::{adapt_model, Display};

#[derive(Display)]
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

serde_for_enum! {
    ParameterExpressionOperator {
        Empty => "empty",
        NotEmpty => "not-empty",
        Equals => "equals",
        NotEquals => "not-equals",
        Less => "less",
        LessEquals => "less-equals",
        More => "more",
        MoreEquals => "more-equals",
        In => "in",
        NotIn => "not-in",
    }
}

#[adapt_model(bdm)]
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
