use crate::{PipelineExecutable, PipelineExecution};
use std::sync::Arc;
use watchmen_model::StdR;
use watchmen_runtime_model_kernel::PipelineSchema;

pub struct CompiledPipeline {
    schema: Arc<PipelineSchema>,
}

impl CompiledPipeline {
    pub fn compile(schema: Arc<PipelineSchema>) -> StdR<Self> {
        Ok(Self { schema })
    }

    pub async fn execute(
        &self,
        executable: PipelineExecutable,
    ) -> StdR<Option<Vec<PipelineExecution>>> {
        todo!("implement execute for CompiledPipeline")
    }
}
