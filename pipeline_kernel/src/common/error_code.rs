use watchmen_model::StdErrorCode;

pub enum PipelineKernelErrorCode {
    EmptyTriggerData,
    TenantIdMissedInTriggerData,
    TenantIdMismatchedWithPrincipal,
    TenantNotExists,
    TriggerTypeNotSupported,
    TriggerTypeNotSupportedOnSynonym,
}

impl StdErrorCode for PipelineKernelErrorCode {
    fn code(&self) -> &'static str {
        match self {
            PipelineKernelErrorCode::EmptyTriggerData => "PLKN-00001",
            PipelineKernelErrorCode::TenantIdMissedInTriggerData => "PLKN-00002",
            PipelineKernelErrorCode::TenantIdMismatchedWithPrincipal => "PLKN-00003",
            PipelineKernelErrorCode::TenantNotExists => "PLKN-00004",
            PipelineKernelErrorCode::TriggerTypeNotSupported => "PLKN-00005",
            PipelineKernelErrorCode::TriggerTypeNotSupportedOnSynonym => "PLKN-00006",
        }
    }
}
