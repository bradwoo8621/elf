use watchmen_base::ErrorCode;

pub enum RuntimeModelKernelErrorCode {
    SnowflakeNodeIdTooBig,
    CannotGetIdGenerator,
    CannotSetIdGenerator,

    TenantIdMissed,
    TenantIdIsBlank,
    NameMissed,
    NameIsBlank,

    TopicIdMissed,
    TopicIdIsBlank,
    TopicTypeMissed,
    TopicKindMissed,
    TopicFactorMissed,

    FactorIdMissed,
    FactorIdIsBlank,
    FactorTypeMissed,

    PipelineIdMissed,
    PipelineIdIsBlank,
    PipelineTypeMissed,
    PipelineStageMissed,
    PipelineUnitMissed,
    PipelineActionMissed,

    ConditionMissed,

    ActionSourceMissed,
    ActionVariableNameMissed,
    ActionVariableNameIsBlank,
    ActionMappingFactorMissed,
    ActionExternalWriterIdMissed,
    ActionExternalWriterIdIsBlank,
    ActionEventCodeMissed,
    ActionEventCodeIsBlank,

    ParameterJointTypeMissed,
    ParameterJointFilterMissed,
    ParameterConstantValueMissed,
    ParameterConstantValueIsBlank,
    ParameterLeftMissed,
    ParameterRightMissed,
    ComputedParametersMissed,
    CaseThenRouteParameterMissed,

    TopicSchemaGenerate,
}

impl ErrorCode for RuntimeModelKernelErrorCode {
    fn code(&self) -> &'static str {
        match self {
            Self::SnowflakeNodeIdTooBig => "RTMK-00001",
            Self::CannotGetIdGenerator => "RTMK-00002",
            Self::CannotSetIdGenerator => "RTMK-00003",

            Self::TenantIdMissed => "RTMK-00100",
            Self::TenantIdIsBlank => "RTMK-00101",
            Self::NameMissed => "RTMK-00102",
            Self::NameIsBlank => "RTMK-00103",
            Self::ConditionMissed => "RTMK-00104",

            Self::TopicIdMissed => "RTMK-00201",
            Self::TopicIdIsBlank => "RTMK-00202",
            Self::TopicTypeMissed => "RTMK-00203",
            Self::TopicKindMissed => "RTMK-00204",
            Self::TopicFactorMissed => "RTMK-00205",
            Self::FactorIdMissed => "RTMK-00206",
            Self::FactorIdIsBlank => "RTMK-00207",
            Self::FactorTypeMissed => "RTMK-00208",

            Self::PipelineIdMissed => "RTMK-00300",
            Self::PipelineIdIsBlank => "RTMK-00301",
            Self::PipelineTypeMissed => "RTMK-00302",
            Self::PipelineStageMissed => "RTMK-00303",
            Self::PipelineUnitMissed => "RTMK-00304",
            Self::PipelineActionMissed => "RTMK-00305",

            Self::ActionSourceMissed => "RTMK-00400",
            Self::ActionVariableNameMissed => "RTMK-00401",
            Self::ActionVariableNameIsBlank => "RTMK-00402",
            Self::ActionMappingFactorMissed => "RTMK-00403",
            Self::ActionExternalWriterIdMissed => "RTMK-00404",
            Self::ActionExternalWriterIdIsBlank => "RTMK-00405",
            Self::ActionEventCodeMissed => "RTMK-00406",
            Self::ActionEventCodeIsBlank => "RTMK-00407",

            Self::ParameterJointTypeMissed => "RTMK-00500",
            Self::ParameterJointFilterMissed => "RTMK-00501",
            Self::ParameterConstantValueMissed => "RTMK-00502",
            Self::ParameterConstantValueIsBlank => "RTMK-00503",
            Self::ParameterLeftMissed => "RTMK-00504",
            Self::ParameterRightMissed => "RTMK-00505",
            Self::ComputedParametersMissed => "RTMK-00506",
            Self::CaseThenRouteParameterMissed => "RTMK-00507",

            Self::TopicSchemaGenerate => "RTMK-00600",
        }
    }
}
