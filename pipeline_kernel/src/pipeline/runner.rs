use crate::{PipelineExecutionRunner, PipelineRunContext};
use tokio;
use watchmen_base::VoidR;

pub struct PipelineRunner;

impl PipelineRunner {
    pub fn execute(context: PipelineRunContext) -> VoidR {
        // TODO currently still on this thread, any needs to create a new thread?
        tokio::spawn(Self::execute_async(context));

        Ok(())
    }

    pub async fn execute_async(mut context: PipelineRunContext) -> VoidR {
        while context.has_more() {
            if let Some(execution) = context.next() {
                context.append(PipelineExecutionRunner::run(execution).await?);
            }
        }

        Ok(())
    }
}
