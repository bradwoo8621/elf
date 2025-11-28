use watchmen_model::StdErrorCode;

pub enum RuntimeModelKernelErrorCode {
    SnowflakeNodeIdTooBig,
    CannotGetIdGenerator,
    CannotSetIdGenerator,
    TopicNameMissed,
    TopicTypeMissed,
    TopicKindMissed,
    TopicTenantMissed
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
        }
    }
}
