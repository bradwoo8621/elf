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
    FullDateTimeParse,
    DateTimeParse,
    DateParse,
    TimeParse,
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
            Self::DecimalParse => "STDE-00001",
            Self::FullDateTimeParse => "STDE-00002",
            Self::DateTimeParse => "STDE-00003",
            Self::DateParse => "STDE-00004",
            Self::TimeParse => "STDE-00005",

            Self::EnvInit => "STDE-00100",
            Self::EnvFileFormatNotSupported => "STDE-00101",
            Self::EnvValueGet => "STDE-00102",
            Self::EnvValueTypeMismatch => "STDE-00103",

            Self::Multiple => "STDE-99998",
            Self::Unknown => "STDE-99999",
        }
    }
}
