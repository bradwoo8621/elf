use watchmen_model::StdErrorCode;

pub enum RuntimeModelKernelErrorCode {
    SnowflakeNodeIdTooBig,
    CannotGetIdGenerator,
    CannotSetIdGenerator,
    TopicNameMissed,
    TopicTypeMissed,
    TopicKindMissed,
    TopicTenantMissed,
    TopicFactorMissed,
    FactorNameMissed,
    FactorTypeMissed,
    TopicIdMissed,
    PipelineIdMissed,
    PipelineTypeMissed,
    PipelineStageMissed,
    PipelineConditionMissed,
    PipelineUnitMissed,
    PipelineActionMissed,
}

impl StdErrorCode for RuntimeModelKernelErrorCode {
    fn code(&self) -> &'static str {
        match self {
            RuntimeModelKernelErrorCode::SnowflakeNodeIdTooBig => "RTMK-00001",
            RuntimeModelKernelErrorCode::CannotGetIdGenerator => "RTMK-00002",
            RuntimeModelKernelErrorCode::CannotSetIdGenerator => "RTMK-00003",
            RuntimeModelKernelErrorCode::TopicNameMissed => "RTMK-00004",
            RuntimeModelKernelErrorCode::TopicTypeMissed => "RTMK-00005",
            RuntimeModelKernelErrorCode::TopicKindMissed => "RTMK-00006",
            RuntimeModelKernelErrorCode::TopicTenantMissed => "RTMK-00007",
            RuntimeModelKernelErrorCode::TopicFactorMissed => "RTMK-00008",
            RuntimeModelKernelErrorCode::FactorNameMissed => "RTMK-00009",
            RuntimeModelKernelErrorCode::FactorTypeMissed => "RTMK-00010",
            RuntimeModelKernelErrorCode::TopicIdMissed => "RTMK-00011",
            RuntimeModelKernelErrorCode::PipelineIdMissed => "RTMK-00012",
            RuntimeModelKernelErrorCode::PipelineTypeMissed => "RTMK-00013",
            RuntimeModelKernelErrorCode::PipelineStageMissed => "RTMK-00014",
            RuntimeModelKernelErrorCode::PipelineConditionMissed => "RTMK-00015",
            RuntimeModelKernelErrorCode::PipelineUnitMissed => "RTMK-00016",
            RuntimeModelKernelErrorCode::PipelineActionMissed => "RTMK-00017",
        }
    }
}
