use crate::{ArcHelper, ArcParameter};
use elf_base::StdR;
use elf_model::{MoreThanExpression, ParameterExpressionOperator};
use std::sync::Arc;

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
