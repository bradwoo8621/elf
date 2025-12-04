use crate::{ArcHelper, ArcParameter};
use std::sync::Arc;
use watchmen_model::{EqualsExpression, ParameterExpressionOperator, StdR};

#[derive(Debug)]
pub struct ArcEqualsExpression {
    pub left: Arc<ArcParameter>,
    pub operator: Arc<ParameterExpressionOperator>,
    pub right: Arc<ArcParameter>,
}

impl ArcHelper for ArcEqualsExpression {}

impl ArcEqualsExpression {
    pub fn new(exp: EqualsExpression) -> StdR<Arc<Self>> {
        let left = Self::parameter_left(exp.left)?;
        let right = Self::parameter_right(exp.right)?;

        Ok(Arc::new(Self {
            left,
            operator: Arc::new(ParameterExpressionOperator::Equals),
            right,
        }))
    }
}
