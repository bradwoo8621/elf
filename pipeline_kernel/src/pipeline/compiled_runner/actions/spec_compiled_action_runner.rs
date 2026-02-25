use crate::{ActionRunResult, CompiledPipeline, CompiledStage, CompiledUnit, InMemoryData};
use elf_auth::Principal;
use std::sync::Arc;

pub trait SpecCompiledActionRunner
where
    Self: Sized,
{
    type SourceCompiledAction;

    fn run(
        in_memory_data: &mut InMemoryData,
        compiled_pipeline: Arc<CompiledPipeline>,
        compiled_stage: Arc<CompiledStage>,
        compiled_unit: Arc<CompiledUnit>,
        action: Arc<Self::SourceCompiledAction>,
        principal: Arc<Principal>,
    ) -> impl Future<Output = ActionRunResult> + Send;
}
