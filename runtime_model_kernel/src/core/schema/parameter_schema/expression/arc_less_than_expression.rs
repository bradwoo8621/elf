use crate::{ArcHelper, ArcParameter};
use elf_base::StdR;
use elf_model::{LessThanExpression, ParameterExpressionOperator};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcLessThanExpression {
    pub left: Arc<ArcParameter>,
    pub operator: Arc<ParameterExpressionOperator>,
    pub right: Arc<ArcParameter>,
}

impl ArcHelper for ArcLessThanExpression {}

impl ArcLessThanExpression {
    pub fn new(exp: LessThanExpression) -> StdR<Arc<Self>> {
        let left = Self::parameter_left(exp.left)?;
        let right = Self::parameter_right(exp.right)?;

        Ok(Arc::new(Self {
            left,
            operator: Arc::new(ParameterExpressionOperator::Less),
            right,
        }))
    }
}
