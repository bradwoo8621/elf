use crate::{CompiledConditional, CompiledStage};
use elf_base::StdR;
use elf_runtime_model_kernel::{PipelineSchema, TopicSchema};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledPipeline {
    topic: Arc<TopicSchema>,
    pipeline: Arc<PipelineSchema>,
    
    conditional: CompiledConditional,
    stages: Vec<CompiledStage>,
}

impl CompiledPipeline {
    pub fn compile(
        topic_schema: Arc<TopicSchema>,
        pipeline_schema: Arc<PipelineSchema>,
    ) -> StdR<Self> {
        // cache all the topic schemas which touched in pipeline compiling phase
        let mut topic_schemas = HashMap::new();
        topic_schemas.insert(topic_schema.topic_id().clone(), topic_schema.clone());

        let pipeline = pipeline_schema.pipeline();
        let compiled_conditional =
            CompiledConditional::compile(&pipeline.on, &mut topic_schemas, &pipeline.tenant_id)?;
        let mut compiled_stages = vec![];
        for stage in pipeline.stages.iter() {
            compiled_stages.push(CompiledStage::compile(
                pipeline,
                stage,
                &mut topic_schemas,
                &pipeline.tenant_id,
            )?);
        }

        Ok(Self {
            topic: topic_schema,
            pipeline: pipeline_schema,
            conditional: compiled_conditional,
            stages: compiled_stages,
        })
    }

    pub fn pipeline_schema(&self) -> &Arc<PipelineSchema> {
        &self.pipeline
    }

    pub fn topic_schema(&self) -> &Arc<TopicSchema> {
        &self.topic
    }

    pub fn conditional(&self) -> &CompiledConditional {
        &self.conditional
    }

    pub fn stages(&self) -> &Vec<CompiledStage> {
        &self.stages
    }
}
