use watchmen_model::StdErrorCode;

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
}

impl StdErrorCode for RuntimeModelKernelErrorCode {
    fn code(&self) -> &'static str {
        match self {
            RuntimeModelKernelErrorCode::SnowflakeNodeIdTooBig => "RTMK-00001",
            RuntimeModelKernelErrorCode::CannotGetIdGenerator => "RTMK-00002",
            RuntimeModelKernelErrorCode::CannotSetIdGenerator => "RTMK-00003",

            RuntimeModelKernelErrorCode::TenantIdMissed => "RTMK-00100",
            RuntimeModelKernelErrorCode::TenantIdIsBlank => "RTMK-00101",
            RuntimeModelKernelErrorCode::NameMissed => "RTMK-00102",
            RuntimeModelKernelErrorCode::NameIsBlank => "RTMK-00103",
            RuntimeModelKernelErrorCode::ConditionMissed => "RTMK-00104",

            RuntimeModelKernelErrorCode::TopicIdMissed => "RTMK-00201",
            RuntimeModelKernelErrorCode::TopicIdIsBlank => "RTMK-00202",
            RuntimeModelKernelErrorCode::TopicTypeMissed => "RTMK-00203",
            RuntimeModelKernelErrorCode::TopicKindMissed => "RTMK-00204",
            RuntimeModelKernelErrorCode::TopicFactorMissed => "RTMK-00205",
            RuntimeModelKernelErrorCode::FactorIdMissed => "RTMK-00206",
            RuntimeModelKernelErrorCode::FactorIdIsBlank => "RTMK-00207",
            RuntimeModelKernelErrorCode::FactorTypeMissed => "RTMK-00208",

            RuntimeModelKernelErrorCode::PipelineIdMissed => "RTMK-00300",
            RuntimeModelKernelErrorCode::PipelineIdIsBlank => "RTMK-00301",
            RuntimeModelKernelErrorCode::PipelineTypeMissed => "RTMK-00302",
            RuntimeModelKernelErrorCode::PipelineStageMissed => "RTMK-00303",
            RuntimeModelKernelErrorCode::PipelineUnitMissed => "RTMK-00304",
            RuntimeModelKernelErrorCode::PipelineActionMissed => "RTMK-00305",

            RuntimeModelKernelErrorCode::ActionSourceMissed => "RTMK-00400",
            RuntimeModelKernelErrorCode::ActionVariableNameMissed => "RTMK-00401",
            RuntimeModelKernelErrorCode::ActionVariableNameIsBlank => "RTMK-00402",
            RuntimeModelKernelErrorCode::ActionMappingFactorMissed => "RTMK-00403",
            RuntimeModelKernelErrorCode::ActionExternalWriterIdMissed => "RTMK-00404",
            RuntimeModelKernelErrorCode::ActionExternalWriterIdIsBlank => "RTMK-00405",
            RuntimeModelKernelErrorCode::ActionEventCodeMissed => "RTMK-00406",
            RuntimeModelKernelErrorCode::ActionEventCodeIsBlank => "RTMK-00407",

            RuntimeModelKernelErrorCode::ParameterJointTypeMissed => "RTMK-00500",
            RuntimeModelKernelErrorCode::ParameterJointFilterMissed => "RTMK-00501",
            RuntimeModelKernelErrorCode::ParameterConstantValueMissed => "RTMK-00502",
            RuntimeModelKernelErrorCode::ParameterConstantValueIsBlank => "RTMK-00503",
            RuntimeModelKernelErrorCode::ParameterLeftMissed => "RTMK-00504",
            RuntimeModelKernelErrorCode::ParameterRightMissed => "RTMK-00505",
            RuntimeModelKernelErrorCode::ComputedParametersMissed => "RTMK-00506",
            RuntimeModelKernelErrorCode::CaseThenRouteParameterMissed => "RTMK-00507",
        }
    }
}
