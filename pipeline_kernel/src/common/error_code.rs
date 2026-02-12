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
    PreviousTopicDataMissed,
    FailedToGetNextSeq,
    // schema
    FactorNotFound,
    // pipeline
    ConstantParameterIsEmpty,
    ConstantParameterIsBlank,
    ComputeParameterTypeMissed,
    ComputeParameterParameterMissed,
    ComputeParameterValueNotSupported,
    ComputeParameterDivideZero,
    ComputeParameterModulusZero,
    ComputeParameterNotADate,
    ActionVariableIsNotPlain,
    UnitLoopVariableMissed,
    // execution
    ExecutionRoundIndexOutOfRange,
    ExecutionRoundHasNoTask,
    ExecutionHasNoRound,
    IncorrectExecutionRoundForAddingTask,
    UnitLoopVariableNotAVec,
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
            Self::PreviousTopicDataMissed => "PLKN-00106",
            Self::FailedToGetNextSeq => "PLKN-00107",

            Self::FactorNotFound => "PLKN-00200",

            Self::ConstantParameterIsEmpty => "PLKN-00300",
            Self::ConstantParameterIsBlank => "PLKN-00301",
            Self::ComputeParameterTypeMissed => "PLKN-00302",
            Self::ComputeParameterParameterMissed => "PLKN-00303",
            Self::ComputeParameterValueNotSupported => "PLKN-00304",
            Self::ComputeParameterDivideZero => "PLKN-00305",
            Self::ComputeParameterModulusZero => "PLKN-00306",
            Self::ComputeParameterNotADate => "PLKN-00307",
            Self::ActionVariableIsNotPlain => "PLKN-00308",
            Self::UnitLoopVariableMissed => "PLKN-00309",

            Self::ExecutionRoundIndexOutOfRange => "PLKN-00400",
            Self::ExecutionRoundHasNoTask => "PLKN-00401",
            Self::ExecutionHasNoRound => "PLKN-00402",
            Self::IncorrectExecutionRoundForAddingTask => "PLKN-00403",
            Self::UnitLoopVariableNotAVec => "PLKN-00404",
        }
    }
}
