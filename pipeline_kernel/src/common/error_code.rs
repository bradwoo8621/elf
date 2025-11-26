use watchmen_model::StdErrorCode;

pub enum PipelineKernelErrorCode {
    TriggerCodeMissed,
    TriggerCodeIsBlank,
    TriggerTypeMissed,
    TriggerTypeNotSupported,
    TriggerTypeNotSupportedOnSynonym,
    TriggerDataMissed,
    TriggerTenantIdMissed,
}

impl StdErrorCode for PipelineKernelErrorCode {
    fn code(&self) -> &'static str {
        match self {
            PipelineKernelErrorCode::TriggerCodeMissed => "PLKN-00001",
            PipelineKernelErrorCode::TriggerCodeIsBlank => "PLKN-00002",
            PipelineKernelErrorCode::TriggerTypeMissed => "PLKN-00003",
            PipelineKernelErrorCode::TriggerTypeNotSupported => "PLKN-00004",
            PipelineKernelErrorCode::TriggerTypeNotSupportedOnSynonym => "PLKN-00005",
            PipelineKernelErrorCode::TriggerDataMissed => "PLKN-00006",
            PipelineKernelErrorCode::TriggerTenantIdMissed => "PLKN-00007",
        }
    }
}
