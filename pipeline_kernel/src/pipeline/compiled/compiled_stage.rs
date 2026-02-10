use crate::{CompiledConditional, CompiledUnit};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{ArcPipeline, ArcPipelineStage, TopicSchema};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledStage {
    conditional: CompiledConditional,
    units: Vec<CompiledUnit>,
}

impl CompiledStage {
    pub fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
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

        Ok(Self {
            conditional: compiled_conditional,
            units: compiled_units,
        })
    }
}
