use watchmen_model::StdErrorCode;

pub enum PipelineKernelErrorCode {
    TriggerCodeMissed,
    TriggerCodeIsBlank,
    TriggerTypeMissed,
    TriggerTypeNotSupported,
    TriggerTypeNotSupportedOnSynonym,
    TriggerDataMissed,
    TriggerTenantIdMissed,
    TriggerTenantIdIsBlank,
    TriggerTenantIdMismatchPrincipal,
    TopicDataIdNotFound,
    TopicDataIdTypeNotSupported,
    TriggerTypeNotSupportedOnRaw,
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
            PipelineKernelErrorCode::TriggerTenantIdIsBlank => "PLKN-00008",
            PipelineKernelErrorCode::TriggerTenantIdMismatchPrincipal => "PLKN-00009",
            PipelineKernelErrorCode::TopicDataIdNotFound => "PLKN-000010",
            PipelineKernelErrorCode::TopicDataIdTypeNotSupported => "PLKN-000011",
            PipelineKernelErrorCode::TriggerTypeNotSupportedOnRaw => "PLKN-000012",
        }
    }
}
