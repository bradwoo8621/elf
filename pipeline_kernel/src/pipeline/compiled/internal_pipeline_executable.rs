use crate::{InMemoryData, PipelineExecutable};
use elf_auth::Principal;
use elf_model::{PipelineMonitorLog, PipelineTriggerTraceId};
use std::sync::Arc;

pub struct InternalPipelineExecutable {
    principal: Arc<Principal>,
    trace_id: Arc<PipelineTriggerTraceId>,

    in_memory_data: InMemoryData,
    log: PipelineMonitorLog,
}

impl InternalPipelineExecutable {
    pub fn create(executable: PipelineExecutable, log: PipelineMonitorLog) -> Self {
        Self {
            principal: executable.principal,
            trace_id: executable.trace_id,

            in_memory_data: InMemoryData::new(executable.variables),
            log,
        }
    }

    pub fn trace_id(&self) -> &Arc<PipelineTriggerTraceId> {
        &self.trace_id
    }

    pub fn principal(&self) -> &Arc<Principal> {
        &self.principal
    }

    pub fn in_memory_data(&mut self) -> &mut InMemoryData {
        &mut self.in_memory_data
    }

    pub fn monitor_log(&mut self) -> &mut PipelineMonitorLog {
        &mut self.log
    }
}
