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
    ValuesNotComparable,
    VariableFuncNotSupported,
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
            PipelineKernelErrorCode::TriggerTypeNotSupportedOnRaw => "PLKN-00010",
            PipelineKernelErrorCode::TriggerPipelineIdIsBlank => "PLKN-00011",
            PipelineKernelErrorCode::TriggerTraceIdIsBlank => "PLKN-00012",
            PipelineKernelErrorCode::TriggerTypeMismatchPipeline => "PLKN-00013",
            PipelineKernelErrorCode::TriggerPipelineNotFound => "PLKN-00014",

            PipelineKernelErrorCode::TopicDataIdNotFound => "PLKN-00100",
            PipelineKernelErrorCode::TopicDataIdTypeNotSupported => "PLKN-00101",
            PipelineKernelErrorCode::CurrentTopicDataMissed => "PLKN-00102",
            PipelineKernelErrorCode::ValuesNotComparable => "PLKN-00103",
            PipelineKernelErrorCode::VariableFuncNotSupported => "PLKN-00104",

            PipelineKernelErrorCode::FactorNotFound => "PLKN-00200",
        }
    }
}
