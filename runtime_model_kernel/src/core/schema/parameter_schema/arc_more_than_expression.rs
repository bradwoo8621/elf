use crate::{ArcHelper, ArcParameter};
use std::sync::Arc;
use watchmen_model::{MoreThanExpression, ParameterExpressionOperator, StdR};

#[derive(Debug)]
pub struct ArcMoreThanExpression {
    pub left: Arc<ArcParameter>,
    pub operator: Arc<ParameterExpressionOperator>,
    pub right: Arc<ArcParameter>,
}

impl ArcHelper for ArcMoreThanExpression {}

impl ArcMoreThanExpression {
    pub fn new(exp: MoreThanExpression) -> StdR<Arc<Self>> {
        let left = Self::parameter_left(exp.left)?;
        let right = Self::parameter_right(exp.right)?;

        Ok(Arc::new(Self {
            left,
            operator: Arc::new(ParameterExpressionOperator::More),
            right,
        }))
    }
}
