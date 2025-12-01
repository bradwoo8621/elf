use std::sync::Arc;
use crate::{ArcHelper, ArcParameter};
use watchmen_model::{InExpression, ParameterExpressionOperator, StdR};

#[derive(Debug)]
pub struct ArcInExpression {
    pub left: Arc<ArcParameter>,
    pub operator: Arc<ParameterExpressionOperator>,
    pub right: Arc<ArcParameter>,
}

impl ArcHelper for ArcInExpression {}

impl ArcInExpression {
    pub fn new(exp: InExpression) -> StdR<Self> {
        let left = Self::parameter_left(exp.left)?;
        let right = Self::parameter_right(exp.right)?;

        Ok(Self {
            left,
            operator: Arc::new(ParameterExpressionOperator::In),
            right,
        })
    }
}
