use crate::{
    create_spec_action_runner, ActionRunResult, CompiledAlarmAction, CompiledPipeline, CompiledStage, CompiledUnit,
    DataPath, InMemoryData, SpecCompiledActionRunner,
};
use chrono::{NaiveDateTime, Utc};
use elf_auth::Principal;
use elf_base::{StdErr, StdR};
use elf_model::{
    ActionMonitorLog, AlarmActionMonitorLog, AlarmActionSeverity, MonitorLogStatus,
    PipelineActionType,
};
use std::ops::Deref;
use std::sync::Arc;

create_spec_action_runner!(CompiledAlarmAction);

impl CompiledAlarmActionRunner {
    fn check_prerequisite(
        &self,
        in_memory_data: &mut InMemoryData,
    ) -> StdR<(bool, Option<(&AlarmActionSeverity, &DataPath)>)> {
        match self.compiled_action.defs() {
            Some((Some(conditional), severity, message)) => Ok((
                conditional.is_true(in_memory_data)?,
                Some((severity, message)),
            )),
            Some((None, severity, message)) => Ok((true, Some((severity, message)))),
            None => Ok((false, None)),
        }
    }

    fn create_monitor_log(&self, prerequisite: bool, error: Option<StdErr>) -> ActionMonitorLog {
        let spent_in_mills =
            (Utc::now().timestamp() - self.start_time.and_utc().timestamp()) as u32;

        let status = if error.is_some() {
            Some(MonitorLogStatus::ERROR)
        } else {
            Some(MonitorLogStatus::DONE)
        };

        ActionMonitorLog::Alarm(AlarmActionMonitorLog {
            action_id: Some(self.compiled_action.action().action_id.deref().clone()),
            r#type: Some(PipelineActionType::Alarm),
            // TODO compute the action defined as
            defined_as: None,
            prerequisite_defined_as: match self.compiled_action.conditional() {
                Some(conditional) => conditional.defined_as(),
                None => None,
            },
            status,
            start_time: Some(self.start_time),
            spent_in_mills: Some(spent_in_mills),
            error: error.map(|e| format!("{}", e)),
            prerequisite: Some(prerequisite),
            // TODO compute touched
            touched: None,
            insert_count: None,
            update_count: None,
            delete_count: None,
        })
    }

    async fn do_run(self, in_memory_data: &mut InMemoryData) -> ActionRunResult {
        match self.check_prerequisite(in_memory_data) {
            Ok((true, Some((severity, message)))) => {
                todo!("implement do_run for CompiledAlarmActionRunner")
            }
            Ok((true, None)) => {
                todo!("implement do_run for CompiledAlarmActionRunner")
            }
            Ok((false, ..)) => {
                todo!("implement do_run for CompiledAlarmActionRunner")
            }
            Err(error) => ActionRunResult {
                created_tasks: None,
                log: self.create_monitor_log(true, Some(error)),
            },
        }
    }
}
