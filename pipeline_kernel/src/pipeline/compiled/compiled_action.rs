use crate::{
    ActionCompiler, CompiledAlarmAction, CompiledCopyToMemoryAction, CompiledDeleteRowAction,
    CompiledDeleteRowsAction, CompiledExistsAction, CompiledInsertOrMergeRowAction,
    CompiledInsertRowAction, CompiledMergeRowAction, CompiledReadFactorAction,
    CompiledReadFactorsAction, CompiledReadRowAction, CompiledReadRowsAction,
    CompiledWriteFactorAction, CompiledWriteToExternalAction,
};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcPipeline, ArcPipelineAction, ArcPipelineStage, ArcPipelineUnit, TopicSchema,
};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub enum CompiledAction {
    Alarm(CompiledAlarmAction),
    CopyToMemory(CompiledCopyToMemoryAction),
    WriteToExternal(CompiledWriteToExternalAction),
    ReadRow(CompiledReadRowAction),
    ReadFactor(CompiledReadFactorAction),
    Exists(CompiledExistsAction),
    ReadRows(CompiledReadRowsAction),
    ReadFactors(CompiledReadFactorsAction),
    MergeRow(CompiledMergeRowAction),
    InsertRow(CompiledInsertRowAction),
    InsertOrMergeRow(CompiledInsertOrMergeRowAction),
    WriteFactor(CompiledWriteFactorAction),
    DeleteRow(CompiledDeleteRowAction),
    DeleteRows(CompiledDeleteRowsAction),
}

struct CompileContext<'a> {
    pipeline: &'a Arc<ArcPipeline>,
    stage: &'a Arc<ArcPipelineStage>,
    unit: &'a Arc<ArcPipelineUnit>,
    topic_schemas: &'a mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
    tenant_id: &'a Arc<TenantId>,
}

impl CompileContext<'_> {
    fn compile<C>(self, action: &C::SourceAction) -> StdR<CompiledAction>
    where
        C: ActionCompiler,
    {
        C::compile(
            self.pipeline,
            self.stage,
            self.unit,
            action,
            self.topic_schemas,
            self.tenant_id,
        )
        .map(|compiled| C::wrap_into_enum(compiled))
    }
}

impl CompiledAction {
    pub fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &Arc<ArcPipelineAction>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        let context = CompileContext {
            pipeline,
            stage,
            unit,
            topic_schemas,
            tenant_id,
        };
        match action.deref() {
            ArcPipelineAction::Alarm(action) => context.compile::<CompiledAlarmAction>(action),
            ArcPipelineAction::CopyToMemory(action) => {
                context.compile::<CompiledCopyToMemoryAction>(action)
            }
            ArcPipelineAction::WriteToExternal(action) => {
                context.compile::<CompiledWriteToExternalAction>(action)
            }
            ArcPipelineAction::ReadRow(action) => context.compile::<CompiledReadRowAction>(action),
            ArcPipelineAction::ReadFactor(action) => {
                context.compile::<CompiledReadFactorAction>(action)
            }
            ArcPipelineAction::Exists(action) => context.compile::<CompiledExistsAction>(action),
            ArcPipelineAction::ReadRows(action) => {
                context.compile::<CompiledReadRowsAction>(action)
            }
            ArcPipelineAction::ReadFactors(action) => {
                context.compile::<CompiledReadFactorsAction>(action)
            }
            ArcPipelineAction::MergeRow(action) => {
                context.compile::<CompiledMergeRowAction>(action)
            }
            ArcPipelineAction::InsertRow(action) => {
                context.compile::<CompiledInsertRowAction>(action)
            }
            ArcPipelineAction::InsertOrMergeRow(action) => {
                context.compile::<CompiledInsertOrMergeRowAction>(action)
            }
            ArcPipelineAction::WriteFactor(action) => {
                context.compile::<CompiledWriteFactorAction>(action)
            }
            ArcPipelineAction::DeleteRow(action) => {
                context.compile::<CompiledDeleteRowAction>(action)
            }
            ArcPipelineAction::DeleteRows(action) => {
                context.compile::<CompiledDeleteRowsAction>(action)
            }
        }
    }
}
