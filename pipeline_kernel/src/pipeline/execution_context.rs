use crate::{
    PipelineExecuteTopicData, PipelineExecutionTask, PipelineExecutionTaskRound,
    PipelineKernelErrorCode,
};
use elf_auth::Principal;
use elf_base::{ErrorCode, StdR, VoidR};
use elf_model::PipelineTriggerTraceId;
use elf_runtime_model_kernel::{PipelineSchema, TopicSchema};
use std::sync::Arc;

/// pipeline execution context
/// there might be multiple tasks are executed, each task represents one pipeline
pub struct PipelineExecutionContext {
    task_rounds: Vec<PipelineExecutionTaskRound>,
}

impl PipelineExecutionContext {
    pub fn create(
        principal: Arc<Principal>,
        topic_data: Arc<PipelineExecuteTopicData>,
        topic_schema: Arc<TopicSchema>,
        pipeline_schemas: Vec<Arc<PipelineSchema>>,
        trace_id: Arc<PipelineTriggerTraceId>,
    ) -> Self {
        let mut round0 = PipelineExecutionTaskRound::new();
        for pipeline_scheme in pipeline_schemas {
            round0.add_task(PipelineExecutionTask::new(
                principal.clone(),
                topic_data.clone(),
                topic_schema.clone(),
                pipeline_scheme,
                trace_id.clone(),
            ));
        }

        Self {
            task_rounds: vec![round0],
        }
    }

    pub fn add_tasks(&mut self, round_index: usize, tasks: Vec<PipelineExecutionTask>) -> VoidR {
        let max_round = self.task_rounds.len();
        if round_index == max_round {
            // round is not created yet
            let mut new_round = PipelineExecutionTaskRound::new();
            new_round.add_tasks(tasks);
            self.task_rounds.push(new_round);
            Ok(())
        } else if round_index == max_round - 1 {
            // round is created already
            if let Some(round) = self.task_rounds.last_mut() {
                round.add_tasks(tasks);
                Ok(())
            } else {
                PipelineKernelErrorCode::ExecutionHasNoRound.msg("No round in context.")
            }
        } else {
            PipelineKernelErrorCode::IncorrectExecutionRoundForAddingTask.msg(format!(
                "Available rounds for adding tasks are {} and {}, currently asked round is {}.",
                max_round - 1,
                max_round,
                round_index
            ))
        }
    }

    pub fn has_more_task(&self) -> bool {
        if let Some(round) = self.task_rounds.last() {
            round.has_task()
        } else {
            false
        }
    }

    /// take next task of given round
    pub fn take_task(&mut self, round_index: usize) -> StdR<Option<PipelineExecutionTask>> {
        let max_round = self.task_rounds.len();
        if round_index >= max_round {
            return PipelineKernelErrorCode::ExecutionRoundIndexOutOfRange.msg(format!(
                "There are [0 - {}] rounds of tasks, asked round is {} and out of range.",
                max_round - 1,
                round_index,
            ));
        }

        if let Some(round) = self.task_rounds.get_mut(round_index) {
            if round.has_task() {
                Ok(round.take_task())
            } else {
                PipelineKernelErrorCode::ExecutionRoundHasNoTask
                    .msg(format!("No task in round[{}].", round_index,))
            }
        } else {
            PipelineKernelErrorCode::ExecutionRoundIndexOutOfRange.msg(format!(
                "There are [0 - {}] rounds of tasks, asked round is {} and out of range.",
                max_round - 1,
                round_index,
            ))
        }
    }
}
