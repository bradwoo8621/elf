use crate::{
	ArcAlarmAction, ArcCopyToMemoryAction, ArcDeleteRowAction, ArcDeleteRowsAction,
	ArcExistsAction, ArcInsertOrMergeRowAction, ArcInsertRowAction, ArcMergeRowAction,
	ArcReadFactorAction, ArcReadFactorsAction, ArcReadRowAction, ArcReadRowsAction,
	ArcWriteFactorAction, ArcWriteToExternalAction,
};
use elf_base::StdR;
use elf_model::PipelineAction;
use std::sync::Arc;

#[derive(Debug)]
pub enum ArcPipelineAction {
    Alarm(ArcAlarmAction),
    CopyToMemory(ArcCopyToMemoryAction),
    WriteToExternal(ArcWriteToExternalAction),
    ReadRow(ArcReadRowAction),
    ReadFactor(ArcReadFactorAction),
    Exists(ArcExistsAction),
    ReadRows(ArcReadRowsAction),
    ReadFactors(ArcReadFactorsAction),
    MergeRow(ArcMergeRowAction),
    InsertRow(ArcInsertRowAction),
    InsertOrMergeRow(ArcInsertOrMergeRowAction),
    WriteFactor(ArcWriteFactorAction),
    DeleteRow(ArcDeleteRowAction),
    DeleteRows(ArcDeleteRowsAction),
}

impl ArcPipelineAction {
    pub fn new(action: PipelineAction) -> StdR<Arc<Self>> {
        let arc_action = match action {
            PipelineAction::Alarm(action) => ArcPipelineAction::Alarm(ArcAlarmAction::new(action)?),
            PipelineAction::CopyToMemory(action) => {
                ArcPipelineAction::CopyToMemory(ArcCopyToMemoryAction::new(action)?)
            }
            PipelineAction::WriteToExternal(action) => {
                ArcPipelineAction::WriteToExternal(ArcWriteToExternalAction::new(action)?)
            }
            PipelineAction::ReadRow(action) => {
                ArcPipelineAction::ReadRow(ArcReadRowAction::new(action)?)
            }
            PipelineAction::ReadFactor(action) => {
                ArcPipelineAction::ReadFactor(ArcReadFactorAction::new(action)?)
            }
            PipelineAction::Exists(action) => {
                ArcPipelineAction::Exists(ArcExistsAction::new(action)?)
            }
            PipelineAction::ReadRows(action) => {
                ArcPipelineAction::ReadRows(ArcReadRowsAction::new(action)?)
            }
            PipelineAction::ReadFactors(action) => {
                ArcPipelineAction::ReadFactors(ArcReadFactorsAction::new(action)?)
            }
            PipelineAction::MergeRow(action) => {
                ArcPipelineAction::MergeRow(ArcMergeRowAction::new(action)?)
            }
            PipelineAction::InsertRow(action) => {
                ArcPipelineAction::InsertRow(ArcInsertRowAction::new(action)?)
            }
            PipelineAction::InsertOrMergeRow(action) => {
                ArcPipelineAction::InsertOrMergeRow(ArcInsertOrMergeRowAction::new(action)?)
            }
            PipelineAction::WriteFactor(action) => {
                ArcPipelineAction::WriteFactor(ArcWriteFactorAction::new(action)?)
            }
            PipelineAction::DeleteRow(action) => {
                ArcPipelineAction::DeleteRow(ArcDeleteRowAction::new(action)?)
            }
            PipelineAction::DeleteRows(action) => {
                ArcPipelineAction::DeleteRows(ArcDeleteRowsAction::new(action)?)
            }
        };

        Ok(Arc::new(arc_action))
    }
}
