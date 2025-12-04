use crate::{ArcHelper, ArcParameter};
use std::sync::Arc;
use watchmen_model::{NotEmptyExpression, ParameterExpressionOperator, StdR};

#[derive(Debug)]
pub struct ArcNotEmptyExpression {
    pub left: Arc<ArcParameter>,
    pub operator: Arc<ParameterExpressionOperator>,
}

impl ArcHelper for ArcNotEmptyExpression {}

impl ArcNotEmptyExpression {
    pub fn new(exp: NotEmptyExpression) -> StdR<Arc<Self>> {
        let left = Self::parameter_left(exp.left)?;

        Ok(Arc::new(Self {
            left,
            operator: Arc::new(ParameterExpressionOperator::NotEmpty),
        }))
    }
}
