use crate::{
    create_spec_action_runner, ActionExecuteLog, ActionRunResult, CompiledAlarmAction, CompiledPipeline,
    CompiledStage, CompiledUnit, DataPath, InMemoryData, SpecCompiledActionRunner,
};
use chrono::{NaiveDateTime, Utc};
use elf_auth::Principal;
use elf_base::{StdErr, StdR};
use elf_model::{AlarmActionSeverity, MonitorLogStatus};
use std::sync::Arc;

create_spec_action_runner!(CompiledAlarmAction);

impl CompiledAlarmActionRunner {
    fn check_prerequisite(
        &self,
        in_memory_data: &mut InMemoryData,
    ) -> StdR<(bool, Option<(&AlarmActionSeverity, &DataPath)>)> {
        match self.compiled_action.fields() {
            Some((Some(conditional), severity, message)) => Ok((
                conditional.is_true(in_memory_data)?,
                Some((severity, message)),
            )),
            Some((None, severity, message)) => Ok((true, Some((severity, message)))),
            None => Ok((false, None)),
        }
    }

    fn create_monitor_log(&self, prerequisite: bool, error: Option<StdErr>) -> ActionExecuteLog {
        let spent_in_mills =
            (Utc::now().timestamp() - self.start_time.and_utc().timestamp()) as u32;

        let status = if error.is_some() {
            MonitorLogStatus::ERROR
        } else {
            MonitorLogStatus::DONE
        };

        ActionExecuteLog {
            action_id: self.compiled_action.action().action_id.clone(),
            r#type: self.compiled_action.action().r#type.clone(),
            // TODO compute the action defined as
            defined_as: None,
            prerequisite_defined_as: match self.compiled_action.conditional() {
                Some(conditional) => conditional.defined_as(),
                None => None,
            },
            status,
            start_time: self.start_time,
            spent_in_mills,
            error: error.map(|e| format!("{}", e)),
            prerequisite,
            find_by: None,
            // TODO compute touched
            touched: None,
            insert_count: None,
            update_count: None,
            delete_count: None,
        }
    }

    async fn do_run(self, in_memory_data: &mut InMemoryData) -> ActionRunResult {
        match self.check_prerequisite(in_memory_data) {
            Ok((true, Some((severity, message)))) => {
                todo!("implement do_run for CompiledAlarmActionRunner")
            }
            Ok((true, None)) => ActionRunResult {
                created_tasks: None,
                log: self.create_monitor_log(true, None),
            },
            Ok((false, ..)) => ActionRunResult {
                created_tasks: None,
                log: self.create_monitor_log(false, None),
            },
            Err(error) => ActionRunResult {
                created_tasks: None,
                log: self.create_monitor_log(false, Some(error)),
            },
        }
    }
}
