use crate::{BaseDataModel, ModelErrorCode, ObjectiveParameter, Storable};
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum, VariousStructTypes};
use serde::{Deserialize, Serialize};

#[derive(Display, Serde, StrEnum)]
pub enum ObjectiveParameterExpressionOperator {
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
pub struct ObjectiveEmptyExpression {
    pub left: Option<ObjectiveParameter>,
    pub operator: Option<ObjectiveParameterExpressionOperator>,
}

impl ObjectiveEmptyExpression {
    pub fn init() -> Self {
        Self::new().operator(ObjectiveParameterExpressionOperator::Empty)
    }

    pub fn to_expression(self) -> ObjectiveParameterExpression {
        ObjectiveParameterExpression::Empty(self)
    }

    pub fn to_condition(self) -> ObjectiveParameterCondition {
        ObjectiveParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveNotEmptyExpression {
    pub left: Option<ObjectiveParameter>,
    pub operator: Option<ObjectiveParameterExpressionOperator>,
}

impl ObjectiveNotEmptyExpression {
    pub fn init() -> Self {
        Self::new().operator(ObjectiveParameterExpressionOperator::NotEmpty)
    }

    pub fn to_expression(self) -> ObjectiveParameterExpression {
        ObjectiveParameterExpression::NotEmpty(self)
    }

    pub fn to_condition(self) -> ObjectiveParameterCondition {
        ObjectiveParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveEqualsExpression {
    pub left: Option<ObjectiveParameter>,
    pub operator: Option<ObjectiveParameterExpressionOperator>,
    pub right: Option<ObjectiveParameter>,
}

impl ObjectiveEqualsExpression {
    pub fn init() -> Self {
        Self::new().operator(ObjectiveParameterExpressionOperator::Equals)
    }

    pub fn to_expression(self) -> ObjectiveParameterExpression {
        ObjectiveParameterExpression::Equals(self)
    }

    pub fn to_condition(self) -> ObjectiveParameterCondition {
        ObjectiveParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveNotEqualsExpression {
    pub left: Option<ObjectiveParameter>,
    pub operator: Option<ObjectiveParameterExpressionOperator>,
    pub right: Option<ObjectiveParameter>,
}

impl ObjectiveNotEqualsExpression {
    pub fn init() -> Self {
        Self::new().operator(ObjectiveParameterExpressionOperator::NotEquals)
    }

    pub fn to_expression(self) -> ObjectiveParameterExpression {
        ObjectiveParameterExpression::NotEquals(self)
    }

    pub fn to_condition(self) -> ObjectiveParameterCondition {
        ObjectiveParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveLessThanExpression {
    pub left: Option<ObjectiveParameter>,
    pub operator: Option<ObjectiveParameterExpressionOperator>,
    pub right: Option<ObjectiveParameter>,
}

impl ObjectiveLessThanExpression {
    pub fn init() -> Self {
        Self::new().operator(ObjectiveParameterExpressionOperator::Less)
    }

    pub fn to_expression(self) -> ObjectiveParameterExpression {
        ObjectiveParameterExpression::LessThan(self)
    }

    pub fn to_condition(self) -> ObjectiveParameterCondition {
        ObjectiveParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveLessThanOrEqualsExpression {
    pub left: Option<ObjectiveParameter>,
    pub operator: Option<ObjectiveParameterExpressionOperator>,
    pub right: Option<ObjectiveParameter>,
}

impl ObjectiveLessThanOrEqualsExpression {
    pub fn init() -> Self {
        Self::new().operator(ObjectiveParameterExpressionOperator::LessEquals)
    }

    pub fn to_expression(self) -> ObjectiveParameterExpression {
        ObjectiveParameterExpression::LessThanOrEquals(self)
    }

    pub fn to_condition(self) -> ObjectiveParameterCondition {
        ObjectiveParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveMoreThanExpression {
    pub left: Option<ObjectiveParameter>,
    pub operator: Option<ObjectiveParameterExpressionOperator>,
    pub right: Option<ObjectiveParameter>,
}

impl ObjectiveMoreThanExpression {
    pub fn init() -> Self {
        Self::new().operator(ObjectiveParameterExpressionOperator::More)
    }

    pub fn to_expression(self) -> ObjectiveParameterExpression {
        ObjectiveParameterExpression::MoreThan(self)
    }

    pub fn to_condition(self) -> ObjectiveParameterCondition {
        ObjectiveParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveMoreThanOrEqualsExpression {
    pub left: Option<ObjectiveParameter>,
    pub operator: Option<ObjectiveParameterExpressionOperator>,
    pub right: Option<ObjectiveParameter>,
}

impl ObjectiveMoreThanOrEqualsExpression {
    pub fn init() -> Self {
        Self::new().operator(ObjectiveParameterExpressionOperator::MoreEquals)
    }

    pub fn to_expression(self) -> ObjectiveParameterExpression {
        ObjectiveParameterExpression::MoreThanOrEquals(self)
    }

    pub fn to_condition(self) -> ObjectiveParameterCondition {
        ObjectiveParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveInExpression {
    pub left: Option<ObjectiveParameter>,
    pub operator: Option<ObjectiveParameterExpressionOperator>,
    pub right: Option<ObjectiveParameter>,
}

impl ObjectiveInExpression {
    pub fn init() -> Self {
        Self::new().operator(ObjectiveParameterExpressionOperator::In)
    }

    pub fn to_expression(self) -> ObjectiveParameterExpression {
        ObjectiveParameterExpression::In(self)
    }

    pub fn to_condition(self) -> ObjectiveParameterCondition {
        ObjectiveParameterCondition::Expression(self.to_expression())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveNotInExpression {
    pub left: Option<ObjectiveParameter>,
    pub operator: Option<ObjectiveParameterExpressionOperator>,
    pub right: Option<ObjectiveParameter>,
}

impl ObjectiveNotInExpression {
    pub fn init() -> Self {
        Self::new().operator(ObjectiveParameterExpressionOperator::NotIn)
    }

    pub fn to_expression(self) -> ObjectiveParameterExpression {
        ObjectiveParameterExpression::NotIn(self)
    }

    pub fn to_condition(self) -> ObjectiveParameterCondition {
        ObjectiveParameterCondition::Expression(self.to_expression())
    }
}

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(tag = "operator")]
pub enum ObjectiveParameterExpression {
    #[serde(rename = "empty")]
    Empty(ObjectiveEmptyExpression),
    #[serde(rename = "not-empty")]
    NotEmpty(ObjectiveNotEmptyExpression),
    #[serde(rename = "equals")]
    Equals(ObjectiveEqualsExpression),
    #[serde(rename = "not-equals")]
    NotEquals(ObjectiveNotEqualsExpression),
    #[serde(rename = "less")]
    LessThan(ObjectiveLessThanExpression),
    #[serde(rename = "less-equals")]
    LessThanOrEquals(ObjectiveLessThanOrEqualsExpression),
    #[serde(rename = "more")]
    MoreThan(ObjectiveMoreThanExpression),
    #[serde(rename = "more-equals")]
    MoreThanOrEquals(ObjectiveMoreThanOrEqualsExpression),
    #[serde(rename = "in")]
    In(ObjectiveInExpression),
    #[serde(rename = "not-in")]
    NotIn(ObjectiveNotInExpression),
}

impl ObjectiveParameterExpression {
    pub fn to_condition(self) -> ObjectiveParameterCondition {
        ObjectiveParameterCondition::Expression(self)
    }
}

#[derive(Display, Serde, StrEnum)]
pub enum ObjectiveParameterJointType {
    And,
    Or,
}

#[adapt_model(storable)]
pub struct ObjectiveParameterJoint {
    pub conj: Option<ObjectiveParameterJointType>, // ObjectiveParameterJointType.AND
    pub filters: Option<Vec<ObjectiveParameterCondition>>,
}

impl ObjectiveParameterJoint {
    pub fn and(filters: Vec<ObjectiveParameterCondition>) -> Self {
        Self {
            conj: Some(ObjectiveParameterJointType::And),
            filters: Some(filters),
        }
    }

    pub fn or(filters: Vec<ObjectiveParameterCondition>) -> Self {
        Self {
            conj: Some(ObjectiveParameterJointType::Or),
            filters: Some(filters),
        }
    }

    pub fn to_condition(self) -> ObjectiveParameterCondition {
        ObjectiveParameterCondition::Joint(self)
    }
}

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(untagged)]
pub enum ObjectiveParameterCondition {
    Expression(ObjectiveParameterExpression),
    Joint(ObjectiveParameterJoint),
}
