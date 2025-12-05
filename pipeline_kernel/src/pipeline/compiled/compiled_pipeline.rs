use crate::{CompiledConditional, PipelineExecutable, PipelineExecution};
use std::sync::Arc;
use watchmen_model::StdR;
use watchmen_runtime_model_kernel::{PipelineSchema, TopicSchema};

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
        let conditional = CompiledConditional::new(&pipeline.on, &pipeline.tenant_id)?;

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
        if self.conditional.is_true(&variables)? {
            // skip the execution because doesn't meet the prerequisite
            Ok(None)
        } else {
            todo!("implement execute for CompiledPipeline")
        }
    }
}
