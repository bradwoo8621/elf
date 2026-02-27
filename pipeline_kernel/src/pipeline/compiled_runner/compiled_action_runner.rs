use crate::{
    ActionExecuteLog, CompiledAction, CompiledAlarmActionRunner, CompiledCopyToMemoryActionRunner,
    CompiledDeleteRowActionRunner, CompiledDeleteRowsActionRunner, CompiledExistsActionRunner,
    CompiledInsertOrMergeRowActionRunner, CompiledInsertRowActionRunner,
    CompiledMergeRowActionRunner, CompiledPipeline, CompiledReadFactorActionRunner,
    CompiledReadFactorsActionRunner, CompiledReadRowActionRunner, CompiledReadRowsActionRunner,
    CompiledStage, CompiledUnit, CompiledWriteFactorActionRunner,
    CompiledWriteToExternalActionRunner, InMemoryData, PipelineExecutionTask,
    SpecCompiledActionRunner,
};
use elf_auth::Principal;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledActionRunner {
    compiled_pipeline: Arc<CompiledPipeline>,
    compiled_stage: Arc<CompiledStage>,
    compiled_unit: Arc<CompiledUnit>,
    principal: Arc<Principal>,
}

pub struct ActionRunResult {
    pub created_tasks: Option<Vec<PipelineExecutionTask>>,
    pub log: ActionExecuteLog,
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
            principal,
        }
        .do_run_action(in_memory_data, compiled_action)
        .await
    }

    async fn do_run_action(
        self,
        in_memory_data: &mut InMemoryData,
        compiled_action: Arc<CompiledAction>,
    ) -> ActionRunResult {
        match compiled_action.deref() {
            CompiledAction::Alarm(action) => {
                self.do_run::<CompiledAlarmActionRunner>(in_memory_data, action.clone())
                    .await
            }
            CompiledAction::CopyToMemory(action) => {
                self.do_run::<CompiledCopyToMemoryActionRunner>(in_memory_data, action.clone())
                    .await
            }
            CompiledAction::WriteToExternal(action) => {
                self.do_run::<CompiledWriteToExternalActionRunner>(in_memory_data, action.clone())
                    .await
            }
            CompiledAction::ReadRow(action) => {
                self.do_run::<CompiledReadRowActionRunner>(in_memory_data, action.clone())
                    .await
            }
            CompiledAction::ReadFactor(action) => {
                self.do_run::<CompiledReadFactorActionRunner>(in_memory_data, action.clone())
                    .await
            }
            CompiledAction::Exists(action) => {
                self.do_run::<CompiledExistsActionRunner>(in_memory_data, action.clone())
                    .await
            }
            CompiledAction::ReadRows(action) => {
                self.do_run::<CompiledReadRowsActionRunner>(in_memory_data, action.clone())
                    .await
            }
            CompiledAction::ReadFactors(action) => {
                self.do_run::<CompiledReadFactorsActionRunner>(in_memory_data, action.clone())
                    .await
            }
            CompiledAction::MergeRow(action) => {
                self.do_run::<CompiledMergeRowActionRunner>(in_memory_data, action.clone())
                    .await
            }
            CompiledAction::InsertRow(action) => {
                self.do_run::<CompiledInsertRowActionRunner>(in_memory_data, action.clone())
                    .await
            }
            CompiledAction::InsertOrMergeRow(action) => {
                self.do_run::<CompiledInsertOrMergeRowActionRunner>(in_memory_data, action.clone())
                    .await
            }
            CompiledAction::WriteFactor(action) => {
                self.do_run::<CompiledWriteFactorActionRunner>(in_memory_data, action.clone())
                    .await
            }
            CompiledAction::DeleteRow(action) => {
                self.do_run::<CompiledDeleteRowActionRunner>(in_memory_data, action.clone())
                    .await
            }
            CompiledAction::DeleteRows(action) => {
                self.do_run::<CompiledDeleteRowsActionRunner>(in_memory_data, action.clone())
                    .await
            }
        }
    }

    async fn do_run<C>(
        self,
        in_memory_data: &mut InMemoryData,
        compiled_action: Arc<C::SourceCompiledAction>,
    ) -> ActionRunResult
    where
        C: SpecCompiledActionRunner,
    {
        C::run(
            in_memory_data,
            self.compiled_pipeline,
            self.compiled_stage,
            self.compiled_unit,
            compiled_action,
            self.principal,
        )
        .await
    }
}
