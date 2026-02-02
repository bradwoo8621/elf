use crate::{ArcHelper, ArcParameter};
use elf_base::StdR;
use elf_model::{NotEmptyExpression, ParameterExpressionOperator};
use std::sync::Arc;

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
