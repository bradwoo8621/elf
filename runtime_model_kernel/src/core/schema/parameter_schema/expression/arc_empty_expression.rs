use crate::{ArcHelper, ArcParameter};
use elf_base::StdR;
use elf_model::{EmptyExpression, ParameterExpressionOperator};
use std::sync::Arc;

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
