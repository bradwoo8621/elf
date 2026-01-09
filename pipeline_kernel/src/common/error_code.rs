use elf_base::ErrorCode;

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
    IncorrectDataPath,
    // schema
    FactorNotFound,
    TopicDataPropertySegmentMissed,
}

impl ErrorCode for PipelineKernelErrorCode {
    fn code(&self) -> &'static str {
        match self {
            Self::TriggerCodeMissed => "PLKN-00001",
            Self::TriggerCodeIsBlank => "PLKN-00002",
            Self::TriggerTypeMissed => "PLKN-00003",
            Self::TriggerTypeNotSupported => "PLKN-00004",
            Self::TriggerTypeNotSupportedOnSynonym => "PLKN-00005",
            Self::TriggerDataMissed => "PLKN-00006",
            Self::TriggerTenantIdMissed => "PLKN-00007",
            Self::TriggerTenantIdIsBlank => "PLKN-00008",
            Self::TriggerTenantIdMismatchPrincipal => "PLKN-00009",
            Self::TriggerTypeNotSupportedOnRaw => "PLKN-00010",
            Self::TriggerPipelineIdIsBlank => "PLKN-00011",
            Self::TriggerTraceIdIsBlank => "PLKN-00012",
            Self::TriggerTypeMismatchPipeline => "PLKN-00013",
            Self::TriggerPipelineNotFound => "PLKN-00014",

            Self::TopicDataIdNotFound => "PLKN-00100",
            Self::TopicDataIdTypeNotSupported => "PLKN-00101",
            Self::CurrentTopicDataMissed => "PLKN-00102",
            Self::ValuesNotComparable => "PLKN-00103",
            Self::VariableFuncNotSupported => "PLKN-00104",
            Self::IncorrectDataPath => "PLKN-00105",

            Self::FactorNotFound => "PLKN-00200",
            Self::TopicDataPropertySegmentMissed => "PLKN-00201",
        }
    }
}
