use crate::{ActionRunResult, CompiledPipeline, CompiledStage, CompiledUnit, InMemoryData};
use elf_auth::Principal;
use std::sync::Arc;

pub trait SpecCompiledActionRunner
where
    Self: Sized,
{
    type SourceCompiledAction;

    fn run(
        in_memory_data: &mut InMemoryData,
        compiled_pipeline: Arc<CompiledPipeline>,
        compiled_stage: Arc<CompiledStage>,
        compiled_unit: Arc<CompiledUnit>,
        compiled_action: Arc<Self::SourceCompiledAction>,
        principal: Arc<Principal>,
    ) -> impl Future<Output = ActionRunResult> + Send;
}

#[macro_export]
macro_rules! create_spec_action_runner {
    ($name:ident) => {
        paste::paste! {
            pub struct [<$name Runner>] {
                compiled_pipeline: Arc<CompiledPipeline>,
                compiled_stage: Arc<CompiledStage>,
                compiled_unit: Arc<CompiledUnit>,
                compiled_action: Arc<$name>,
                principal: Arc<Principal>,

                start_time: NaiveDateTime,
            }

            impl SpecCompiledActionRunner for [<$name Runner>] {
                type SourceCompiledAction = $name;

                async fn run(
                    in_memory_data: &mut InMemoryData,
                    compiled_pipeline: Arc<CompiledPipeline>,
                    compiled_stage: Arc<CompiledStage>,
                    compiled_unit: Arc<CompiledUnit>,
                    compiled_action: Arc<Self::SourceCompiledAction>,
                    principal: Arc<Principal>,
                ) -> ActionRunResult {
                    Self {
                        compiled_pipeline,
                        compiled_stage,
                        compiled_unit,
                        compiled_action,
                        principal,
                        start_time: Utc::now().naive_utc(),
                    }
                    .do_run(in_memory_data)
                    .await
                }
            }
        }
    };
}
