use crate::{
	AlarmActionMonitorLog, CopyToMemoryActionMonitorLog, DeleteActionMonitorLog, MonitorLogStatus,
	ReadActionMonitorLog, WriteActionMonitorLog, WriteToExternalActionMonitorLog,
};
use elf_model_marco::VariousStructTypes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(untagged)]
pub enum ActionMonitorLog {
    Alarm(AlarmActionMonitorLog),
    CopyToMemory(CopyToMemoryActionMonitorLog),
    WriteToExternal(WriteToExternalActionMonitorLog),
    Read(ReadActionMonitorLog),
    Write(WriteActionMonitorLog),
    Delete(DeleteActionMonitorLog),
}

impl ActionMonitorLog {
    pub fn status(&self) -> &Option<MonitorLogStatus> {
        match self {
            ActionMonitorLog::Alarm(AlarmActionMonitorLog { status, .. }) => status,
            ActionMonitorLog::CopyToMemory(CopyToMemoryActionMonitorLog { status, .. }) => status,
            ActionMonitorLog::WriteToExternal(WriteToExternalActionMonitorLog { status, .. }) => status,
            ActionMonitorLog::Write(WriteActionMonitorLog { status, .. }) => status,
            ActionMonitorLog::Read(ReadActionMonitorLog { status, .. }) => status,
            ActionMonitorLog::Delete(DeleteActionMonitorLog { status, .. }) => status,
        }
    }
}
