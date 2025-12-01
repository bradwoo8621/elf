use crate::{ArcHelper, ArcParameter};
use std::sync::Arc;
use watchmen_model::{LessThanExpression, ParameterExpressionOperator, StdR};

#[derive(Debug)]
pub struct ArcLessThanExpression {
    pub left: Arc<ArcParameter>,
    pub operator: Arc<ParameterExpressionOperator>,
    pub right: Arc<ArcParameter>,
}

impl ArcHelper for ArcLessThanExpression {}

impl ArcLessThanExpression {
    pub fn new(exp: LessThanExpression) -> StdR<Self> {
        let left = Self::parameter_left(exp.left)?;
        let right = Self::parameter_right(exp.right)?;

        Ok(Self {
            left,
            operator: Arc::new(ParameterExpressionOperator::Less),
            right,
        })
    }
}
