use std::sync::Arc;
use watchmen_model::{PipelineAction, StdR};

#[derive(Debug)]
pub struct ArcPipelineAction {}

impl ArcPipelineAction {
    pub fn new(_action: PipelineAction) -> StdR<Arc<Self>> {
        // TODO
        Ok(Arc::new(Self {}))
    }
}
