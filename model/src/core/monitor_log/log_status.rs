use crate::ModelErrorCode;
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
#[pattern = "upper-case"]
pub enum MonitorLogStatus {
    /// even step is ignored by prerequisite is false, it is treated as DONE
    DONE,
    /// step never be touched
    IGNORED,
    /// exception occurred
    ERROR,
}
