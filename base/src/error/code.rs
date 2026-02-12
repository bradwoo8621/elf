use crate::{StdErr, StdErrDetail, StdR};
use std::panic::Location;

pub trait ErrorCode {
    fn code(&self) -> &'static str;

    #[track_caller]
    fn msg<R, M>(&self, msg: M) -> StdR<R>
    where
        M: Into<String>,
    {
        StdErr::of_with_location(self.code(), msg.into(), Location::caller())
    }

    #[track_caller]
    fn err<R>(&self) -> StdR<R> {
        StdErr::code_only_with_location(self.code(), Location::caller())
    }

    #[track_caller]
    fn e_msg<M>(&self, msg: M) -> StdErr
    where
        M: Into<String>,
    {
        StdErr::create(
            self.code(),
            Some(StdErrDetail::Str(msg.into())),
            Location::caller(),
        )
    }

    #[track_caller]
    fn e(&self) -> StdErr {
        StdErr::create(self.code(), None, Location::caller())
    }
}

pub enum StdErrCode {
    DecimalParse,
    FullDateTimeParse,
    DateTimeParse,
    DateParse,
    TimeParse,
    StrParse,
    DateMovementParse,
    /// environment variables
    EnvInit,
    EnvFileFormatNotSupported,
    EnvValueGet,
    EnvValueTypeMismatch,

    RwLock,
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
            Self::StrParse => "STDE-00006",
            Self::DateMovementParse => "STDE-00007",

            Self::EnvInit => "STDE-00100",
            Self::EnvFileFormatNotSupported => "STDE-00101",
            Self::EnvValueGet => "STDE-00102",
            Self::EnvValueTypeMismatch => "STDE-00103",

            Self::RwLock => "STDE-99997",
            Self::Multiple => "STDE-99998",
            Self::Unknown => "STDE-99999",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ErrorCode, StdErrCode};

    #[test]
    fn test() {
        let r: Result<String, _> = StdErrCode::Unknown.msg("Unknown error.");
        if r.is_err() {
            println!("{}", r.err().unwrap())
        }
    }
}
