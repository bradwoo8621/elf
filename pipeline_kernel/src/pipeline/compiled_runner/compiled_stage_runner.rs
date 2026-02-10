use crate::{CompiledPipeline, CompiledStage, InMemoryData, PipelineExecutionTask};
use chrono::{NaiveDateTime, Utc};
use elf_auth::Principal;
use elf_model::StageMonitorLog;

pub struct CompiledStageRunner<'a> {
    in_memory_data: &'a InMemoryData,

    compiled_pipeline: &'a CompiledPipeline,
    compiled_stage: &'a CompiledStage,
    principal: &'a Principal,

    start_time: NaiveDateTime,
}

pub struct StageRunResult {
    pub created_tasks: Option<Vec<PipelineExecutionTask>>,
    pub log: StageMonitorLog,
}

impl<'a> CompiledStageRunner<'a> {
    pub async fn run(
        in_memory_data: &'a mut InMemoryData,
        compiled_pipeline: &'a CompiledPipeline,
        compiled_stage: &'a CompiledStage,
        principal: &'a Principal,
    ) -> StageRunResult {
        Self {
            in_memory_data,
            compiled_pipeline,
            compiled_stage,
            principal,

            start_time: Utc::now().naive_utc(),
        }
        .do_run()
        .await
    }

    async fn do_run(mut self) -> StageRunResult {
        todo!("implement do_run for CompiledStageRunner")
    }
}
