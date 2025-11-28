use std::sync::Arc;
use watchmen_model::{ParameterJoint, StdR};

#[derive(Debug)]
pub struct ArcParameterJoint {}

impl ArcParameterJoint {
    pub fn new(_joint: ParameterJoint) -> StdR<Arc<ArcParameterJoint>> {
        // TODO
        Ok(Arc::new(Self {}))
    }
}
