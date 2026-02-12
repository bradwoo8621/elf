use crate::{CompiledAction, CompiledConditional};
use elf_base::{StdR, StringUtils};
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{ArcPipeline, ArcPipelineStage, ArcPipelineUnit, TopicSchema};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledUnit {
    pipeline: Arc<ArcPipeline>,
    stage: Arc<ArcPipelineStage>,
    unit: Arc<ArcPipelineUnit>,

    conditional: CompiledConditional,
    has_loop: bool,
    loop_variable_name: Option<String>,
    actions: Vec<CompiledAction>,
}

impl CompiledUnit {
    pub fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        let compiled_conditional =
            CompiledConditional::compile(&unit.on, topic_schemas, tenant_id)?;
        let (has_loop, loop_variable_name) =
            if let Some(loop_variable_name) = &unit.loop_variable_name {
                if loop_variable_name.is_not_blank() {
                    (true, Some(loop_variable_name.deref().clone()))
                } else {
                    (false, None)
                }
            } else {
                (false, None)
            };
        let mut compiled_actions = vec![];
        for action in unit.r#do.iter() {
            compiled_actions.push(CompiledAction::compile(
                pipeline,
                stage,
                unit,
                action,
                topic_schemas,
                tenant_id,
            )?);
        }

        Ok(Self {
            pipeline: pipeline.clone(),
            stage: stage.clone(),
            unit: unit.clone(),

            conditional: compiled_conditional,
            has_loop,
            loop_variable_name,
            actions: compiled_actions,
        })
    }

    pub fn pipeline(&self) -> &Arc<ArcPipeline> {
        &self.pipeline
    }

    pub fn stage(&self) -> &Arc<ArcPipelineStage> {
        &self.stage
    }

    pub fn unit(&self) -> &Arc<ArcPipelineUnit> {
        &self.unit
    }

    pub fn has_loop(&self) -> bool {
        self.has_loop
    }

    pub fn loop_variable_name(&self) -> &Option<String> {
        &self.loop_variable_name
    }

    pub fn conditional(&self) -> &CompiledConditional {
        &self.conditional
    }

    pub fn actions(&self) -> &Vec<CompiledAction> {
        &self.actions
    }
}
