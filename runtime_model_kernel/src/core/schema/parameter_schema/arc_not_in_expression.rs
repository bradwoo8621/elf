use crate::{ArcHelper, ArcParameter};
use std::sync::Arc;
use watchmen_model::{NotInExpression, ParameterExpressionOperator, StdR};

#[derive(Debug)]
pub struct ArcNotInExpression {
    pub left: Arc<ArcParameter>,
    pub operator: Arc<ParameterExpressionOperator>,
    pub right: Arc<ArcParameter>,
}

impl ArcHelper for ArcNotInExpression {}

impl ArcNotInExpression {
    pub fn new(exp: NotInExpression) -> StdR<Self> {
        let left = Self::parameter_left(exp.left)?;
        let right = Self::parameter_right(exp.right)?;

        Ok(Self {
            left,
            operator: Arc::new(ParameterExpressionOperator::Equals),
            right,
        })
    }
}
