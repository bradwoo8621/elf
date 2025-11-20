use crate::{BaseDataModel, Parameter, Storable};
use serde::{Deserialize, Serialize};
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
pub struct EmptyExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
}

#[adapt_model(storable)]
pub struct NotEmptyExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
}

#[adapt_model(storable)]
pub struct EqualsExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
    pub right: Option<Parameter>,
}

#[adapt_model(storable)]
pub struct NotEqualsExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
    pub right: Option<Parameter>,
}

#[adapt_model(storable)]
pub struct LessThanExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
    pub right: Option<Parameter>,
}

#[adapt_model(storable)]
pub struct LessThanOrEqualsExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
    pub right: Option<Parameter>,
}

#[adapt_model(storable)]
pub struct MoreThanExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
    pub right: Option<Parameter>,
}

#[adapt_model(storable)]
pub struct MoreThanOrEqualsExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
    pub right: Option<Parameter>,
}

#[adapt_model(storable)]
pub struct InExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
    pub right: Option<Parameter>,
}

#[adapt_model(storable)]
pub struct NotInExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
    pub right: Option<Parameter>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "operator")]
pub enum ParameterExpression {
    #[serde(rename = "empty")]
    Empty(EmptyExpression),
    #[serde(rename = "not-empty")]
    NotEmpty(NotEmptyExpression),
    #[serde(rename = "equals")]
    Equals(EqualsExpression),
    #[serde(rename = "not-equals")]
    NotEquals(NotEqualsExpression),
    #[serde(rename = "less")]
    LessThan(LessThanExpression),
    #[serde(rename = "less-equals")]
    LessThanOrEquals(LessThanOrEqualsExpression),
    #[serde(rename = "more")]
    MoreThan(MoreThanExpression),
    #[serde(rename = "more-equals")]
    MoreThanOrEqual(MoreThanOrEqualsExpression),
    #[serde(rename = "in")]
    In(InExpression),
    #[serde(rename = "not-in")]
    NotIn(NotInExpression),
}
