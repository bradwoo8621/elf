use crate::{CompiledConditional, CompiledUnit};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{ArcPipeline, ArcPipelineStage, TopicSchema};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledStage {
    pipeline: Arc<ArcPipeline>,
    stage: Arc<ArcPipelineStage>,

    conditional: CompiledConditional,
    units: Vec<Arc<CompiledUnit>>,
}

impl CompiledStage {
    pub fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Arc<Self>> {
        let compiled_conditional =
            CompiledConditional::compile(&stage.on, topic_schemas, tenant_id)?;
        let mut compiled_units = vec![];
        for unit in stage.units.iter() {
            compiled_units.push(CompiledUnit::compile(
                pipeline,
                stage,
                unit,
                topic_schemas,
                tenant_id,
            )?);
        }

        Ok(Arc::new(Self {
            pipeline: pipeline.clone(),
            stage: stage.clone(),

            conditional: compiled_conditional,
            units: compiled_units,
        }))
    }

    pub fn pipeline(&self) -> &Arc<ArcPipeline> {
        &self.pipeline
    }

    pub fn stage(&self) -> &Arc<ArcPipelineStage> {
        &self.stage
    }

    pub fn conditional(&self) -> &CompiledConditional {
        &self.conditional
    }

    pub fn units(&self) -> &Vec<Arc<CompiledUnit>> {
        &self.units
    }
}
