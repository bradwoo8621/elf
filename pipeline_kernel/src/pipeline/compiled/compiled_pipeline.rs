use crate::{CompiledConditional, InMemoryData, PipelineExecutable, PipelineExecution};
use elf_base::StdR;
use elf_runtime_model_kernel::{PipelineSchema, TopicSchema};
use std::sync::Arc;

pub struct CompiledPipeline {
    topic: Arc<TopicSchema>,
    pipeline: Arc<PipelineSchema>,
    conditional: CompiledConditional,
}

impl CompiledPipeline {
    pub fn compile(
        topic_schema: Arc<TopicSchema>,
        pipeline_schema: Arc<PipelineSchema>,
    ) -> StdR<Self> {
        let pipeline = pipeline_schema.pipeline();
        let conditional = CompiledConditional::compile(&pipeline.on, &pipeline.tenant_id)?;

        Ok(Self {
            topic: topic_schema,
            pipeline: pipeline_schema,
            conditional,
        })
    }

    pub async fn execute(
        &self,
        executable: PipelineExecutable,
    ) -> StdR<Option<Vec<PipelineExecution>>> {
        let variables = executable.variables;
        let mut in_memory_data = InMemoryData::new(&variables);
        if self.conditional.is_true(&mut in_memory_data)? {
            // skip the execution because doesn't meet the prerequisite
            Ok(None)
        } else {
            todo!("implement execute for CompiledPipeline")
        }
    }
}
