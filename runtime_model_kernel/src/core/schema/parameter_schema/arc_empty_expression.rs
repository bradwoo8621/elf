use crate::{ArcHelper, ArcParameter};
use std::sync::Arc;
use watchmen_model::{EmptyExpression, ParameterExpressionOperator, StdR};

#[derive(Debug)]
pub struct ArcEmptyExpression {
    pub left: Arc<ArcParameter>,
    pub operator: Arc<ParameterExpressionOperator>,
}

impl ArcHelper for ArcEmptyExpression {}

impl ArcEmptyExpression {
    pub fn new(exp: EmptyExpression) -> StdR<Arc<Self>> {
        let left = Self::parameter_left(exp.left)?;

        Ok(Arc::new(Self {
            left,
            operator: Arc::new(ParameterExpressionOperator::Empty),
        }))
    }
}
