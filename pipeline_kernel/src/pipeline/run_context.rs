use crate::{PipelineExecution, PipelineTrigger, TopicTrigger};
use std::collections::VecDeque;
use std::sync::Arc;
use watchmen_runtime_model_kernel::PipelineSchema;

/// It includes at least one pipeline execution,
/// and based on the execution results of the already included pipelines,
/// the number of pipelines included in the entire execution context may increase
/// because some pipelines can trigger other pipelines.
pub struct PipelineRunContext {
    queue: VecDeque<PipelineExecution>,
}

impl PipelineRunContext {
    pub fn new(
        pipeline_trigger: &PipelineTrigger,
        topic_trigger: Arc<TopicTrigger>,
        pipelines: Vec<Arc<PipelineSchema>>,
    ) -> Self {
        PipelineRunContext {
            queue: pipelines
                .into_iter()
                .map(|pipeline_schema| PipelineExecution {
                    topic_schema: pipeline_trigger.topic_schema.clone(),
                    topic_trigger: topic_trigger.clone(),
                    pipeline_schema,
                    // env
                    principal: pipeline_trigger.principal.clone(),
                    trace_id: pipeline_trigger.trace_id.clone(),
                    execution_log_monitor: pipeline_trigger.execution_log_monitor.clone(),
                })
                .collect(),
        }
    }

    pub fn next(&mut self) -> Option<PipelineExecution> {
        self.queue.pop_front()
    }

    pub fn has_more(&self) -> bool {
        !self.queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn append(&mut self, executions: Option<Vec<PipelineExecution>>) {
        if let Some(executions) = executions {
            executions.into_iter().for_each(|e| self.queue.push_back(e))
        }
    }
}
