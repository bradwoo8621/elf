use crate::{ArcHelper, ArcParameter};
use elf_base::StdR;
use elf_model::{CopyToMemoryAction, PipelineActionId, PipelineActionType};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcCopyToMemoryAction {
    pub action_id: Arc<PipelineActionId>,
    pub r#type: Arc<PipelineActionType>,
    pub source: Arc<ArcParameter>,
    pub variable_name: Arc<String>,
}

impl ArcHelper for ArcCopyToMemoryAction {}

impl ArcCopyToMemoryAction {
    pub fn new(action: CopyToMemoryAction) -> StdR<Self> {
        let action_id = Self::or_empty_str(action.action_id);
        let source = Self::action_source(action.source, || {
            format!("Copy to memory action[{}]", action_id)
        })?;
        let variable_name = Self::variable_name(action.variable_name, || {
            format!("Copy to memory action[{}]", action_id)
        })?;

        Ok(Self {
            action_id,
            r#type: Arc::new(PipelineActionType::CopyToMemory),
            source,
            variable_name,
        })
    }
}
