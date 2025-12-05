use watchmen_model::StdErrorCode;

pub enum PipelineKernelErrorCode {
    // trigger
    TriggerCodeMissed,
    TriggerCodeIsBlank,
    TriggerTypeMissed,
    TriggerTypeNotSupported,
    TriggerTypeNotSupportedOnSynonym,
    TriggerDataMissed,
    TriggerTenantIdMissed,
    TriggerTenantIdIsBlank,
    TriggerTenantIdMismatchPrincipal,
    TriggerTypeNotSupportedOnRaw,
    TriggerPipelineIdIsBlank,
    TriggerTraceIdIsBlank,
    TriggerTypeMismatchPipeline,
    TriggerPipelineNotFound,
    // topic data
    TopicDataIdNotFound,
    TopicDataIdTypeNotSupported,
    CurrentTopicDataMissed,
    // schema
    FactorNotFound,
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
            PipelineKernelErrorCode::TriggerTypeNotSupportedOnRaw => "PLKN-000010",
            PipelineKernelErrorCode::TriggerPipelineIdIsBlank => "PLKN-000011",
            PipelineKernelErrorCode::TriggerTraceIdIsBlank => "PLKN-000012",
            PipelineKernelErrorCode::TriggerTypeMismatchPipeline => "PLKN-000013",
            PipelineKernelErrorCode::TriggerPipelineNotFound => "PLKN-000014",

            PipelineKernelErrorCode::TopicDataIdNotFound => "PLKN-000100",
            PipelineKernelErrorCode::TopicDataIdTypeNotSupported => "PLKN-000101",
            PipelineKernelErrorCode::CurrentTopicDataMissed => "PLKN-000102",

            PipelineKernelErrorCode::FactorNotFound => "PLKN-000200",
        }
    }
}
