use crate::{
    CompiledPipeline, CompiledStage, CompiledUnitRunner, InMemoryData, PipelineExecutionTask,
    UnitRunResult,
};
use chrono::{NaiveDateTime, Utc};
use elf_auth::Principal;
use elf_base::{StdErr, StdR};
use elf_model::{MonitorLogStatus, StageMonitorLog, UnitMonitorLog};
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledStageRunner {
    compiled_pipeline: Arc<CompiledPipeline>,
    compiled_stage: Arc<CompiledStage>,
    principal: Arc<Principal>,

    start_time: NaiveDateTime,
}

pub struct StageRunResult {
    pub created_tasks: Option<Vec<PipelineExecutionTask>>,
    pub log: StageMonitorLog,
}

impl CompiledStageRunner {
    pub async fn run(
        in_memory_data: &mut InMemoryData,
        compiled_pipeline: Arc<CompiledPipeline>,
        compiled_stage: Arc<CompiledStage>,
        principal: Arc<Principal>,
    ) -> StageRunResult {
        Self {
            compiled_pipeline,
            compiled_stage,
            principal,

            start_time: Utc::now().naive_utc(),
        }
        .do_run(in_memory_data)
        .await
    }

    fn check_prerequisite(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        self.compiled_stage.conditional().is_true(in_memory_data)
    }

    fn create_monitor_log(
        &self,
        prerequisite: bool,
        unit_logs: Option<Vec<UnitMonitorLog>>,
        error: Option<StdErr>,
    ) -> StageMonitorLog {
        let spent_in_mills =
            (Utc::now().timestamp() - self.start_time.and_utc().timestamp()) as u32;

        StageMonitorLog {
            stage_id: Some(self.compiled_stage.stage().stage_id.deref().clone()),
            name: Some(self.compiled_stage.stage().name.deref().clone()),
            prerequisite_defined_as: self.compiled_stage.conditional().defined_as(),
            status: Some(match &error {
                Some(_) => MonitorLogStatus::DONE,
                None => MonitorLogStatus::ERROR,
            }),
            start_time: Some(self.start_time),
            spent_in_mills: Some(spent_in_mills),
            error: error.map(|e| format!("{}", e)),
            prerequisite: Some(prerequisite),
            units: unit_logs,
        }
    }

    async fn do_run(self, in_memory_data: &mut InMemoryData) -> StageRunResult {
        match self.check_prerequisite(in_memory_data) {
            Ok(true) => {
                let mut all_unit_accomplished: bool = true;
                let mut unit_logs = vec![];
                let mut created_tasks = vec![];
                for unit in self.compiled_stage.units().iter() {
                    let results = CompiledUnitRunner::run(
                        in_memory_data,
                        self.compiled_pipeline.clone(),
                        self.compiled_stage.clone(),
                        unit.clone(),
                        self.principal.clone(),
                    )
                    .await;

                    for UnitRunResult {
                        created_tasks: created_tasks_by_unit,
                        log,
                    } in results
                    {
                        // push created tasks by stage into created tasks
                        if let Some(created_tasks_by_unit) = created_tasks_by_unit {
                            created_tasks.extend(created_tasks_by_unit);
                        }
                        // check there is any error occurred in stage running
                        let has_error = match log.status {
                            Some(MonitorLogStatus::ERROR) => true,
                            _ => false,
                        };
                        unit_logs.push(log);

                        if has_error {
                            all_unit_accomplished = false;
                            break;
                        }
                    }
                }

                StageRunResult {
                    created_tasks: Some(created_tasks),
                    log: self.create_monitor_log(all_unit_accomplished, Some(unit_logs), None),
                }
            }
            Ok(false) => StageRunResult {
                created_tasks: None,
                log: self.create_monitor_log(false, None, None),
            },
            Err(error) => StageRunResult {
                created_tasks: None,
                log: self.create_monitor_log(true, None, Some(error)),
            },
        }
    }
}
