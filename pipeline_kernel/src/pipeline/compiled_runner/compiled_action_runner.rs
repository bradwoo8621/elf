use crate::{
    CompiledAction, CompiledPipeline, CompiledStage, CompiledUnit, InMemoryData, UnitRunResult,
};
use chrono::{NaiveDateTime, Utc};
use elf_auth::Principal;

pub struct CompiledActionRunner<'a> {
    in_memory_data: &'a InMemoryData,

    compiled_pipeline: &'a CompiledPipeline,
    compiled_stage: &'a CompiledStage,
    compiled_unit: &'a CompiledUnit,
    compiled_action: &'a CompiledAction,
    principal: &'a Principal,

    start_time: NaiveDateTime,
}

impl<'a> CompiledActionRunner<'a> {
    pub async fn run(
        in_memory_data: &'a mut InMemoryData,
        compiled_pipeline: &'a CompiledPipeline,
        compiled_stage: &'a CompiledStage,
        compiled_unit: &'a CompiledUnit,
        compiled_action: &'a CompiledAction,
        principal: &'a Principal,
    ) -> UnitRunResult {
        Self {
            in_memory_data,
            compiled_pipeline,
            compiled_stage,
            compiled_unit,
            compiled_action,
            principal,

            start_time: Utc::now().naive_utc(),
        }
        .do_run()
        .await
    }

    async fn do_run(mut self) -> UnitRunResult {
        todo!("implement do_run for CompiledActionRunner")
    }
}
