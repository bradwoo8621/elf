use crate::{ArcHelper, ArcParameter};
use std::sync::Arc;
use watchmen_model::{NotEqualsExpression, ParameterExpressionOperator, StdR};

#[derive(Debug)]
pub struct ArcNotEqualsExpression {
    pub left: Arc<ArcParameter>,
    pub operator: Arc<ParameterExpressionOperator>,
    pub right: Arc<ArcParameter>,
}

impl ArcHelper for ArcNotEqualsExpression {}

impl ArcNotEqualsExpression {
    pub fn new(exp: NotEqualsExpression) -> StdR<Arc<Self>> {
        let left = Self::parameter_left(exp.left)?;
        let right = Self::parameter_right(exp.right)?;

        Ok(Arc::new(Self {
            left,
            operator: Arc::new(ParameterExpressionOperator::NotEquals),
            right,
        }))
    }
}
