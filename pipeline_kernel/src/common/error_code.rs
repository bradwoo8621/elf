use watchmen_model::StdErrorCode;

pub enum PipelineKernelErrorCode {
    EmptyTriggerData,
    InvalidTriggerData,
}

impl StdErrorCode for PipelineKernelErrorCode {
    fn code(&self) -> &'static str {
        match self {
            PipelineKernelErrorCode::EmptyTriggerData => "PLKN-00001",
            PipelineKernelErrorCode::InvalidTriggerData => "PLKN-00002",
        }
    }
}
