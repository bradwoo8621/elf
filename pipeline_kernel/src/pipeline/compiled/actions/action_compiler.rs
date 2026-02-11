use crate::CompiledAction;
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{ArcPipeline, ArcPipelineStage, ArcPipelineUnit, TopicSchema};
use std::collections::HashMap;
use std::sync::Arc;

pub trait ActionCompiler
where
    Self: Sized,
{
    type SourceAction;

    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &Self::SourceAction,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self>;

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction;
}
