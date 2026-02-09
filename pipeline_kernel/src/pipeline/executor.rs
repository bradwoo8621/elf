use crate::{PipelineExecuteRequest, PipelineExecutionContext, PipelineExecutionTaskRunner};
use elf_base::{StdR, VoidR};
use elf_model::TopicDataId;
use std::ops::Deref;

enum BuiltContext {
    Context((TopicDataId, PipelineExecutionContext)),
    NoContext(TopicDataId),
}

pub struct PipelineExecutor;

impl PipelineExecutor {
    fn build_context(request: PipelineExecuteRequest) -> StdR<BuiltContext> {
        let topic_data_id = request.topic_data_id();
        let topic_schema = request.topic_schema();
        let context = request.create_execution_context();
        if !context.has_more_task() {
            println!(
                "No pipeline needs to be triggered by topic[topic_id={}, topic_name={}].",
                topic_schema.topic_id(),
                topic_schema.name()
            );
            Ok(BuiltContext::NoContext(topic_data_id.deref().clone()))
        } else {
            Ok(BuiltContext::Context((
                topic_data_id.deref().clone(),
                context,
            )))
        }
    }

    /// - execute task one-by-one.
    /// - tasks created by task are added into next round
    /// - all tasks are finished in [round n], then start tasks in [round n + 1].
    ///
    /// TODO maybe tasks in same round can be run parallel,
    ///  in parallel situation, how to control the context
    ///  since it might be modified by any task (add created new tasks into context)
    async fn do_execute_async(mut context: PipelineExecutionContext) -> VoidR {
        let mut round_index = 0;
        while context.has_more_task() {
            if let Some(task) = context.take_task(round_index)? {
                if let Some(more_tasks) = PipelineExecutionTaskRunner::run_async(task).await? {
                    // this task created more tasks
                    context.add_tasks(round_index + 1, more_tasks)?;
                }
            } else {
                // all task of this round consumed
                round_index += 1;
            }
        }

        Ok(())
    }

    pub fn execute(request: PipelineExecuteRequest) -> StdR<TopicDataId> {
        match PipelineExecutor::build_context(request)? {
            BuiltContext::NoContext(topic_data_id) => Ok(topic_data_id),
            BuiltContext::Context((topic_data_id, context)) => {
                // TODO how spawn doing here, and how to configure the Runtime?
                tokio::spawn(Self::do_execute_async(context));

                Ok(topic_data_id)
            }
        }
    }

    pub async fn execute_async(request: PipelineExecuteRequest) -> StdR<TopicDataId> {
        match PipelineExecutor::build_context(request)? {
            BuiltContext::NoContext(topic_data_id) => Ok(topic_data_id),
            BuiltContext::Context((topic_data_id, context)) => {
                Self::do_execute_async(context).await?;
                Ok(topic_data_id)
            }
        }
    }
}
