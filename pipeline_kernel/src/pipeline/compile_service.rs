use crate::CompiledPipeline;
use elf_base::StdR;
use elf_runtime_model_kernel::{PipelineSchema, PipelineService, TopicSchema};
use std::sync::Arc;

pub struct PipelineCompileService;

impl PipelineCompileService {
    fn new() -> StdR<Arc<Self>> {
        // TODO maybe find from cache
        Ok(Arc::new(Self {}))
    }

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
