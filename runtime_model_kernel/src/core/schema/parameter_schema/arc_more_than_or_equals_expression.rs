use crate::{ArcHelper, ArcParameter};
use std::sync::Arc;
use watchmen_model::{MoreThanOrEqualsExpression, ParameterExpressionOperator, StdR};

#[derive(Debug)]
pub struct ArcMoreThanOrEqualsExpression {
    pub left: Arc<ArcParameter>,
    pub operator: Arc<ParameterExpressionOperator>,
    pub right: Arc<ArcParameter>,
}

impl ArcHelper for ArcMoreThanOrEqualsExpression {}

impl ArcMoreThanOrEqualsExpression {
    pub fn new(exp: MoreThanOrEqualsExpression) -> StdR<Arc<Self>> {
        let left = Self::parameter_left(exp.left)?;
        let right = Self::parameter_right(exp.right)?;

        Ok(Arc::new(Self {
            left,
            operator: Arc::new(ParameterExpressionOperator::MoreEquals),
            right,
        }))
    }
}
