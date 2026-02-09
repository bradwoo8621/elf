use crate::{
	AlarmActionMonitorLog, CopyToMemoryActionMonitorLog, DeleteActionMonitorLog,
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
