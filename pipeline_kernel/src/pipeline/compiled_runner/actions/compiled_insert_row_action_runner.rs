use crate::{
    create_spec_action_runner, ActionRunResult, CompiledInsertRowAction, CompiledPipeline, CompiledStage,
    CompiledUnit, InMemoryData, SpecCompiledActionRunner,
};
use chrono::{NaiveDateTime, Utc};
use elf_auth::Principal;
use std::sync::Arc;

create_spec_action_runner!(CompiledInsertRowAction);

impl CompiledInsertRowActionRunner {
    async fn do_run(self, in_memory_data: &mut InMemoryData) -> ActionRunResult {
        todo!("implement do_run for CompiledInsertRowActionRunner")
    }
}
