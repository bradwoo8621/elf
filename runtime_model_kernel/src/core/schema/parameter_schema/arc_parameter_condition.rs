use crate::{ArcParameterExpression, ArcParameterJoint};
use std::sync::Arc;
use watchmen_model::{ParameterCondition, StdR};

#[derive(Debug)]
pub enum ArcParameterCondition {
    Expression(Arc<ArcParameterExpression>),
    Joint(Arc<ArcParameterJoint>),
}

impl ArcParameterCondition {
    pub fn new(condition: ParameterCondition) -> StdR<Arc<Self>> {
        let arc_parameter = match condition {
            ParameterCondition::Expression(p) => {
                ArcParameterCondition::Expression(ArcParameterExpression::new(p)?)
            }
            ParameterCondition::Joint(p) => {
                ArcParameterCondition::Joint(ArcParameterJoint::new(p)?)
            }
        };

        Ok(Arc::new(arc_parameter))
    }
}
