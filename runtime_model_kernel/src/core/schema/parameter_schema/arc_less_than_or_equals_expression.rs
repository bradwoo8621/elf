use crate::{ArcHelper, ArcParameter};
use elf_base::StdR;
use elf_model::{LessThanOrEqualsExpression, ParameterExpressionOperator};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcLessThanOrEqualsExpression {
    pub left: Arc<ArcParameter>,
    pub operator: Arc<ParameterExpressionOperator>,
    pub right: Arc<ArcParameter>,
}

impl ArcHelper for ArcLessThanOrEqualsExpression {}

impl ArcLessThanOrEqualsExpression {
    pub fn new(exp: LessThanOrEqualsExpression) -> StdR<Arc<Self>> {
        let left = Self::parameter_left(exp.left)?;
        let right = Self::parameter_right(exp.right)?;

        Ok(Arc::new(Self {
            left,
            operator: Arc::new(ParameterExpressionOperator::LessEquals),
            right,
        }))
    }
}
