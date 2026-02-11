use crate::{CompiledAction, CompiledConditional, DataPath};
use elf_base::{StdR, StringUtils};
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{ArcPipeline, ArcPipelineStage, ArcPipelineUnit, TopicSchema};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledUnit {
    conditional: CompiledConditional,
    has_loop: bool,
    loop_variable_path: Option<DataPath>,
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
        let (has_loop, loop_variable_path) =
            if let Some(loop_variable_name) = &unit.loop_variable_name {
                if loop_variable_name.is_not_blank() {
                    (true, Some(DataPath::from_str(loop_variable_name)?))
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
            conditional: compiled_conditional,
            has_loop,
            loop_variable_path,
            actions: compiled_actions,
        })
    }
}
