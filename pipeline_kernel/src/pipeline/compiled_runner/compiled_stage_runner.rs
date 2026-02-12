use crate::{
    CompiledPipeline, CompiledStage, CompiledUnitRunner, InMemoryData, PipelineExecutionTask,
    UnitRunResult,
};
use chrono::{NaiveDateTime, Utc};
use elf_auth::Principal;
use elf_base::{StdErr, StdR};
use elf_model::{MonitorLogStatus, StageMonitorLog, UnitMonitorLog};
use std::ops::Deref;

pub struct CompiledStageRunner<'a> {
    in_memory_data: &'a mut InMemoryData,

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

    fn check_prerequisite(&mut self) -> StdR<bool> {
        self.compiled_stage
            .conditional()
            .is_true(self.in_memory_data)
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
            prerequisite_defined_as: self.compiled_pipeline.conditional().defined_as(),
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

    async fn do_run(mut self) -> StageRunResult {
        match self.check_prerequisite() {
            Ok(true) => {
                let mut all_unit_accomplished: bool = true;
                let mut unit_logs = vec![];
                let mut created_tasks = vec![];
                for unit in self.compiled_stage.units().iter() {
                    let UnitRunResult {
                        created_tasks: created_tasks_by_unit,
                        log,
                    } = CompiledUnitRunner::run(
                        &mut self.in_memory_data,
                        self.compiled_pipeline,
                        self.compiled_stage,
                        unit,
                        self.principal,
                    )
                    .await;

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
