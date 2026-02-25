use crate::{
    ActionRunResult, CompiledMergeRowAction, CompiledPipeline, CompiledStage, CompiledUnit,
    InMemoryData, SpecCompiledActionRunner,
};
use chrono::{NaiveDateTime, Utc};
use elf_auth::Principal;
use std::sync::Arc;

pub struct CompiledMergeRowActionRunner {
    compiled_pipeline: Arc<CompiledPipeline>,
    compiled_stage: Arc<CompiledStage>,
    compiled_unit: Arc<CompiledUnit>,
    compiled_action: Arc<CompiledMergeRowAction>,
    principal: Arc<Principal>,

    start_time: NaiveDateTime,
}

impl CompiledMergeRowActionRunner {
    async fn do_run(self, in_memory_data: &mut InMemoryData) -> ActionRunResult {
        todo!("implement do_run for CompiledMergeRowActionRunner")
    }
}

impl SpecCompiledActionRunner for CompiledMergeRowActionRunner {
    type SourceCompiledAction = CompiledMergeRowAction;

    async fn run(
        in_memory_data: &mut InMemoryData,
        compiled_pipeline: Arc<CompiledPipeline>,
        compiled_stage: Arc<CompiledStage>,
        compiled_unit: Arc<CompiledUnit>,
        compiled_action: Arc<Self::SourceCompiledAction>,
        principal: Arc<Principal>,
    ) -> ActionRunResult {
        Self {
            compiled_pipeline,
            compiled_stage,
            compiled_unit,
            compiled_action,
            principal,
            start_time: Utc::now().naive_utc(),
        }
        .do_run(in_memory_data)
        .await
    }
}
