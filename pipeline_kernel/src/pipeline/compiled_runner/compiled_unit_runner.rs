use crate::{
	CompiledPipeline, CompiledStage, CompiledUnit, InMemoryData,
	PipelineExecutionTask,
};
use chrono::{NaiveDateTime, Utc};
use elf_auth::Principal;
use elf_model::UnitMonitorLog;

pub struct CompiledUnitRunner<'a> {
    in_memory_data: &'a InMemoryData,

    compiled_pipeline: &'a CompiledPipeline,
    compiled_stage: &'a CompiledStage,
    compiled_unit: &'a CompiledUnit,
    principal: &'a Principal,

    start_time: NaiveDateTime,
}

pub struct UnitRunResult {
    pub created_tasks: Option<Vec<PipelineExecutionTask>>,
    pub log: UnitMonitorLog,
}

impl<'a> CompiledUnitRunner<'a> {
    pub async fn run(
        in_memory_data: &'a mut InMemoryData,
        compiled_pipeline: &'a CompiledPipeline,
        compiled_stage: &'a CompiledStage,
        compiled_unit: &'a CompiledUnit,
        principal: &'a Principal,
    ) -> UnitRunResult {
        Self {
            in_memory_data,
            compiled_pipeline,
            compiled_stage,
            compiled_unit,
            principal,

            start_time: Utc::now().naive_utc(),
        }
        .do_run()
        .await
    }

    async fn do_run(mut self) -> UnitRunResult {
        todo!("implement do_run for CompiledUnitRunner")
    }
}
