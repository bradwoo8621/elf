use crate::{
    ActionRunResult, CompiledDeleteRowsAction, CompiledPipeline, CompiledStage, CompiledUnit,
    InMemoryData, SpecCompiledActionRunner,
};
use chrono::{NaiveDateTime, Utc};
use elf_auth::Principal;
use std::sync::Arc;

pub struct CompiledDeleteRowsActionRunner {
    compiled_pipeline: Arc<CompiledPipeline>,
    compiled_stage: Arc<CompiledStage>,
    compiled_unit: Arc<CompiledUnit>,
    compiled_action: Arc<CompiledDeleteRowsAction>,
    principal: Arc<Principal>,

    start_time: NaiveDateTime,
}

impl CompiledDeleteRowsActionRunner {
    async fn do_run(self, in_memory_data: &mut InMemoryData) -> ActionRunResult {
        todo!("implement do_run for CompiledDeleteRowsActionRunner")
    }
}

impl SpecCompiledActionRunner for CompiledDeleteRowsActionRunner {
    type SourceCompiledAction = CompiledDeleteRowsAction;

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
