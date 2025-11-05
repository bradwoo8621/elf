use crate::serde_for_enum;
use crate::BaseDataModel;
use crate::Parameter;
use std::fmt;

pub enum ParameterExpressionOperator {
    Empty,
    NotEmpty,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    MoreThan,
    MoreThanOrEqual,
    In,
    NotIn,
}

impl fmt::Display for ParameterExpressionOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParameterExpressionOperator::Empty => write!(f, "empty"),
            ParameterExpressionOperator::NotEmpty => write!(f, "not-empty"),
            ParameterExpressionOperator::Equal => write!(f, "equals"),
            ParameterExpressionOperator::NotEqual => write!(f, "not-equals"),
            ParameterExpressionOperator::LessThan => write!(f, "less"),
            ParameterExpressionOperator::LessThanOrEqual => write!(f, "less-equals"),
            ParameterExpressionOperator::MoreThan => write!(f, "more"),
            ParameterExpressionOperator::MoreThanOrEqual => write!(f, "more-equals"),
            ParameterExpressionOperator::In => write!(f, "in"),
            ParameterExpressionOperator::NotIn => write!(f, "not-in"),
        }
    }
}

serde_for_enum! {
    ParameterExpressionOperator {
        Empty => "empty",
        NotEmpty => "not-empty",
        Equal => "equals",
        NotEqual => "not-equals",
        LessThan => "less",
        LessThanOrEqual => "less-equals",
        MoreThan => "more",
        MoreThanOrEqual => "more-equals",
        In => "in",
        NotIn => "not-in",
    }
}

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
            ParameterExpression::Equal(_, _) => ParameterExpressionOperator::Equal,
            ParameterExpression::NotEqual(_, _) => ParameterExpressionOperator::NotEqual,
            ParameterExpression::LessThan(_, _) => ParameterExpressionOperator::LessThan,
            ParameterExpression::LessThanOrEqual(_, _) => {
                ParameterExpressionOperator::LessThanOrEqual
            }
            ParameterExpression::MoreThan(_, _) => ParameterExpressionOperator::MoreThan,
            ParameterExpression::MoreThanOrEqual(_, _) => {
                ParameterExpressionOperator::MoreThanOrEqual
            }
            ParameterExpression::In(_) => ParameterExpressionOperator::In,
            ParameterExpression::NotIn(_) => ParameterExpressionOperator::NotIn,
        }
    }
}

impl BaseDataModel for ParameterExpression {}
