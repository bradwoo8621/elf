use crate::{StdErr, StdErrDetail, StdR};

pub trait ErrorCode {
    fn code(&self) -> &'static str;

    fn msg<R, M>(&self, msg: M) -> StdR<R>
    where
        M: Into<String>,
    {
        StdErr::of(self.code(), msg.into())
    }

    fn err<R>(&self) -> StdR<R> {
        StdErr::code_only(self.code())
    }

    fn e_msg<M>(&self, msg: M) -> StdErr
    where
        M: Into<String>,
    {
        StdErr {
            code: self.code(),
            details: Some(StdErrDetail::Str(msg.into())),
        }
    }

    fn e(&self) -> StdErr {
        StdErr {
            code: self.code(),
            details: None,
        }
    }
}

pub enum StdErrCode {
    DecimalParse,
    /// environment variables
    EnvInit,
    EnvFileFormatNotSupported,
    EnvValueGet,
    EnvValueTypeMismatch,
    /// with multiple sub errors
    Multiple,
    Unknown,
}

impl ErrorCode for StdErrCode {
    fn code(&self) -> &'static str {
        match self {
            StdErrCode::DecimalParse => "STDE-00001",

            StdErrCode::EnvInit => "STDE-00100",
            StdErrCode::EnvFileFormatNotSupported => "STDE-00101",
            StdErrCode::EnvValueGet => "STDE-00102",
            StdErrCode::EnvValueTypeMismatch => "STDE-00103",

            StdErrCode::Multiple => "STDE-99998",
            StdErrCode::Unknown => "STDE-99999",
        }
    }
}
