use crate::{
    ArcTopicDataValue, CompiledPipeline, CompiledStage, CompiledUnit, InMemoryData,
    MonitorLogHelper, PipelineExecutionTask, PipelineKernelErrorCode,
};
use chrono::{NaiveDateTime, Utc};
use elf_auth::Principal;
use elf_base::{ErrorCode, StdErr};
use elf_model::{ActionMonitorLog, MonitorLogStatus, UnitMonitorLog};
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledUnitRunner<'a> {
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
    ) -> Vec<UnitRunResult> {
        Self {
            compiled_pipeline,
            compiled_stage,
            compiled_unit,
            principal,

            start_time: Utc::now().naive_utc(),
        }
        .do_run(in_memory_data)
        .await
    }

    fn create_monitor_log(
        &self,
        prerequisite: bool,
        loop_variable_value: Option<Arc<ArcTopicDataValue>>,
        action_logs: Option<Vec<ActionMonitorLog>>,
        error: Option<StdErr>,
    ) -> UnitMonitorLog {
        let spent_in_mills =
            (Utc::now().timestamp() - self.start_time.and_utc().timestamp()) as u32;

        UnitMonitorLog {
            unit_id: Some(self.compiled_unit.stage().stage_id.deref().clone()),
            name: Some(self.compiled_unit.stage().name.deref().clone()),
            loop_variable_name: self.compiled_unit.loop_variable_name().clone(),
            prerequisite_defined_as: self.compiled_unit.conditional().defined_as(),
            status: Some(match &error {
                Some(_) => MonitorLogStatus::DONE,
                None => MonitorLogStatus::ERROR,
            }),
            start_time: Some(self.start_time),
            spent_in_mills: Some(spent_in_mills),
            error: error.map(|e| format!("{}", e)),
            prerequisite: Some(prerequisite),
            loop_variable_value: loop_variable_value.map(|v| MonitorLogHelper::transform_value(&v)),
            actions: action_logs,
        }
    }

    async fn do_run_with_loop_element(
        &self,
        in_memory_data: &mut InMemoryData,
        loop_variable_value: &ArcTopicDataValue,
    ) -> UnitRunResult {
        todo!("implement do_run_round for CompiledUnitRunner")
    }

    async fn do_run_no_loop(&self, in_memory_data: &mut InMemoryData) -> UnitRunResult {
        todo!("implement do_run_round for CompiledUnitRunner")
    }

    fn get_loop_variable(
        &self,
        in_memory_data: &InMemoryData,
        loop_variable_name: &String,
    ) -> Option<Arc<ArcTopicDataValue>> {
        in_memory_data
            .get_variables()
            .get(loop_variable_name)
            .map(|v| v.clone())
    }

    async fn do_run(self, in_memory_data: &mut InMemoryData) -> Vec<UnitRunResult> {
        if self.compiled_unit.has_loop() {
            if let Some(loop_variable_name) = self.compiled_unit.loop_variable_name() {
                let loop_variable_value =
                    self.get_loop_variable(in_memory_data, loop_variable_name);
                match loop_variable_value {
                    None => {
                        vec![UnitRunResult {
                            created_tasks: None,
                            log: self.create_monitor_log(true, None, None, None),
                        }]
                    }
                    Some(value) => {
                        match value.deref() {
                            ArcTopicDataValue::None => {
                                vec![UnitRunResult {
                                    created_tasks: None,
                                    log: self.create_monitor_log(true, None, None, None),
                                }]
                            }
                            ArcTopicDataValue::Vec(vec) => {
                                if vec.is_empty() {
                                    vec![UnitRunResult {
                                        created_tasks: None,
                                        log: self.create_monitor_log(true, Some(value), None, None),
                                    }]
                                } else {
                                    let mut results = vec![];
                                    for element in vec.iter() {
                                        results.push(
                                            self.do_run_with_loop_element(in_memory_data, element)
                                                .await,
                                        );
                                    }
                                    results
                                }
                            }
                            other_value => {
                                // only vec and none are supported, raise error
                                vec![UnitRunResult {
                                    created_tasks: None,
                                    log: self.create_monitor_log(
                                        true,
                                        Some(value.clone()),
                                        None,
                                        Some(PipelineKernelErrorCode::UnitLoopVariableNotAVec.e_msg(format!(
                                            "Loop variable[{}] value of action[action_id={}] must be a list, current is [{}].",
                                            loop_variable_name, self.compiled_unit.unit().unit_id, other_value
                                        ))),
                                    ),
                                }]
                            }
                        }
                    }
                }
            } else {
                // never happen
                vec![UnitRunResult {
                    created_tasks: None,
                    log: self.create_monitor_log(
                        true,
                        None,
                        None,
                        Some(
                            PipelineKernelErrorCode::UnitLoopVariableMissed.e_msg(format!(
                                "Variable name of unit[unit_id={}] is missed.",
                                self.compiled_unit.unit().unit_id
                            )),
                        ),
                    ),
                }]
            }
        } else {
            let result = self.do_run_no_loop(in_memory_data).await;
            vec![result]
        }
    }
}
