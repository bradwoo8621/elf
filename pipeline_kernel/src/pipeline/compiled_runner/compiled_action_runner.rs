use crate::{
    CompiledAction, CompiledPipeline, CompiledStage, CompiledUnit, InMemoryData,
    PipelineExecutionTask,
};
use chrono::{NaiveDateTime, Utc};
use elf_auth::Principal;
use elf_model::{
    ActionMonitorLog, AlarmActionMonitorLog, CopyToMemoryActionMonitorLog, DeleteActionMonitorLog,
    MonitorLogStatus, ReadActionMonitorLog, WriteActionMonitorLog, WriteToExternalActionMonitorLog,
};
use std::sync::Arc;

pub struct CompiledActionRunner {
    compiled_pipeline: Arc<CompiledPipeline>,
    compiled_stage: Arc<CompiledStage>,
    compiled_unit: Arc<CompiledUnit>,
    compiled_action: Arc<CompiledAction>,
    principal: Arc<Principal>,

    start_time: NaiveDateTime,
}

pub struct ActionRunResult {
    pub created_tasks: Option<Vec<PipelineExecutionTask>>,
    pub log: ActionMonitorLog,
}

impl ActionRunResult {
    pub fn has_error(log: &ActionMonitorLog) -> bool {
        let status = match log {
            ActionMonitorLog::Alarm(AlarmActionMonitorLog { status, .. }) => status,
            ActionMonitorLog::CopyToMemory(CopyToMemoryActionMonitorLog { status, .. }) => status,
            ActionMonitorLog::WriteToExternal(WriteToExternalActionMonitorLog {
                status, ..
            }) => status,
            ActionMonitorLog::Write(WriteActionMonitorLog { status, .. }) => status,
            ActionMonitorLog::Read(ReadActionMonitorLog { status, .. }) => status,
            ActionMonitorLog::Delete(DeleteActionMonitorLog { status, .. }) => status,
        };
        match status {
            Some(MonitorLogStatus::ERROR) => true,
            _ => false,
        }
    }
}

impl CompiledActionRunner {
    pub async fn run(
        in_memory_data: &mut InMemoryData,
        compiled_pipeline: Arc<CompiledPipeline>,
        compiled_stage: Arc<CompiledStage>,
        compiled_unit: Arc<CompiledUnit>,
        compiled_action: Arc<CompiledAction>,
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

    async fn do_run(self, in_memory_data: &mut InMemoryData) -> ActionRunResult {
        todo!("implement do_run for CompiledActionRunner")
    }
}
