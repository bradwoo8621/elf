use crate::CompiledPipeline;
use elf_base::StdR;
use elf_runtime_model_kernel::{PipelineSchema, PipelineService, TopicSchema};
use std::sync::Arc;

pub struct PipelineCompileService;

impl PipelineCompileService {
    /// TODO maybe find from cache
    fn new() -> StdR<Arc<Self>> {
        Ok(Arc::new(Self {}))
    }

    /// TODO compiled pipeline maybe find from cache
    pub fn compile(
        &self,
        topic_schema: Arc<TopicSchema>,
        pipeline_schema: Arc<PipelineSchema>,
    ) -> StdR<Arc<CompiledPipeline>> {
        Ok(Arc::new(CompiledPipeline::compile(
            topic_schema,
            pipeline_schema,
        )?))
    }
}

pub trait PipelineCompilationProvider {
    fn compilation() -> StdR<Arc<PipelineCompileService>> {
        PipelineCompileService::new()
    }
}

impl PipelineCompilationProvider for PipelineService {}
