use crate::{BaseDataModel, Parameter, ParameterCondition, Storable};
use serde::{Deserialize, Serialize};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum, VariousStructTypes};

#[derive(Display, Serde, Debug, StrEnum)]
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

impl EmptyExpression {
    pub fn init() -> Self {
        Self::new().operator(ParameterExpressionOperator::Empty)
    }

    pub fn to_expression(self) -> ParameterExpression {
        ParameterExpression::Empty(self)
    }

    pub fn to_condition(self) -> ParameterCondition {
        ParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct NotEmptyExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
}

impl NotEmptyExpression {
    pub fn init() -> Self {
        Self::new().operator(ParameterExpressionOperator::NotEmpty)
    }

    pub fn to_expression(self) -> ParameterExpression {
        ParameterExpression::NotEmpty(self)
    }

    pub fn to_condition(self) -> ParameterCondition {
        ParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct EqualsExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
    pub right: Option<Parameter>,
}

impl EqualsExpression {
    pub fn init() -> Self {
        Self::new().operator(ParameterExpressionOperator::Equals)
    }

    pub fn to_expression(self) -> ParameterExpression {
        ParameterExpression::Equals(self)
    }

    pub fn to_condition(self) -> ParameterCondition {
        ParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct NotEqualsExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
    pub right: Option<Parameter>,
}

impl NotEqualsExpression {
    pub fn init() -> Self {
        Self::new().operator(ParameterExpressionOperator::NotEquals)
    }

    pub fn to_expression(self) -> ParameterExpression {
        ParameterExpression::NotEquals(self)
    }

    pub fn to_condition(self) -> ParameterCondition {
        ParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct LessThanExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
    pub right: Option<Parameter>,
}

impl LessThanExpression {
    pub fn init() -> Self {
        Self::new().operator(ParameterExpressionOperator::Less)
    }

    pub fn to_expression(self) -> ParameterExpression {
        ParameterExpression::LessThan(self)
    }

    pub fn to_condition(self) -> ParameterCondition {
        ParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct LessThanOrEqualsExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
    pub right: Option<Parameter>,
}

impl LessThanOrEqualsExpression {
    pub fn init() -> Self {
        Self::new().operator(ParameterExpressionOperator::LessEquals)
    }

    pub fn to_expression(self) -> ParameterExpression {
        ParameterExpression::LessThanOrEquals(self)
    }

    pub fn to_condition(self) -> ParameterCondition {
        ParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct MoreThanExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
    pub right: Option<Parameter>,
}

impl MoreThanExpression {
    pub fn init() -> Self {
        Self::new().operator(ParameterExpressionOperator::More)
    }

    pub fn to_expression(self) -> ParameterExpression {
        ParameterExpression::MoreThan(self)
    }

    pub fn to_condition(self) -> ParameterCondition {
        ParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct MoreThanOrEqualsExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
    pub right: Option<Parameter>,
}

impl MoreThanOrEqualsExpression {
    pub fn init() -> Self {
        Self::new().operator(ParameterExpressionOperator::MoreEquals)
    }

    pub fn to_expression(self) -> ParameterExpression {
        ParameterExpression::MoreThanOrEquals(self)
    }

    pub fn to_condition(self) -> ParameterCondition {
        ParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct InExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
    pub right: Option<Parameter>,
}

impl InExpression {
    pub fn init() -> Self {
        Self::new().operator(ParameterExpressionOperator::In)
    }

    pub fn to_expression(self) -> ParameterExpression {
        ParameterExpression::In(self)
    }

    pub fn to_condition(self) -> ParameterCondition {
        ParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct NotInExpression {
    pub left: Option<Parameter>,
    pub operator: Option<ParameterExpressionOperator>,
    pub right: Option<Parameter>,
}

impl NotInExpression {
    pub fn init() -> Self {
        Self::new().operator(ParameterExpressionOperator::NotIn)
    }

    pub fn to_expression(self) -> ParameterExpression {
        ParameterExpression::NotIn(self)
    }

    pub fn to_condition(self) -> ParameterCondition {
        ParameterCondition::Expression(self.to_expression())
    }
}

#[derive(Serialize, Deserialize, VariousStructTypes)]
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
    MoreThanOrEquals(MoreThanOrEqualsExpression),
    #[serde(rename = "in")]
    In(InExpression),
    #[serde(rename = "not-in")]
    NotIn(NotInExpression),
}

impl ParameterExpression {
    pub fn to_condition(self) -> ParameterCondition {
        ParameterCondition::Expression(self)
    }
}
