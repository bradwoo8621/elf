use crate::ArcPipeline;
use std::sync::Arc;
use watchmen_model::{Pipeline, StdR};

pub struct PipelineSchema {
    inner: Arc<ArcPipeline>,
}

impl PipelineSchema {
    pub fn new(pipeline: Pipeline) -> StdR<Self> {
        Ok(PipelineSchema {
            inner: ArcPipeline::new(pipeline)?,
        })
    }

    pub fn pipeline(&self) -> &Arc<ArcPipeline> {
        &self.inner
    }

    pub fn pipeline_name(&self) -> Arc<String> {
        self.pipeline().name.clone()
    }
}
