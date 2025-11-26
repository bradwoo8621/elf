use serde::Serialize;

pub trait StdErrorCode {
    fn code(&self) -> &'static str;
}

pub enum StdErrCode {
    DecimalParse,
    /// with multiple sub errors
    Multiple,
    Unknown,
}

impl StdErrorCode for StdErrCode {
    fn code(&self) -> &'static str {
        match self {
            StdErrCode::DecimalParse => "STDE-00001",
            StdErrCode::Multiple => "STDE-99998",
            StdErrCode::Unknown => "STDE-99999",
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum StdErrDetails {
    Str(String),
    Sub(Vec<StdErr>),
}

/// Convert other types of exceptions to this exception to enable the use of the `?` syntactic sugar.
#[derive(Serialize)]
pub struct StdErr {
    /// code must be [XXXX-99999], each module has its own code prefix [XXXX]
    code: &'static str,
    details: Option<StdErrDetails>,
}

impl StdErr {
    pub fn of<R, M>(code: &'static str, msg: M) -> Result<R, Self>
    where
        M: Into<String>,
    {
        Err(StdErr {
            code,
            details: Some(StdErrDetails::Str(msg.into())),
        })
    }

    /// code only
    pub fn co<R>(code: &'static str) -> Result<R, Self> {
        Err(StdErr {
            code,
            details: None,
        })
    }

    /// message only
    pub fn mo<R, M>(msg: M) -> Result<R, Self>
    where
        M: Into<String>,
    {
        Err(StdErr {
            code: StdErrCode::Unknown.code(),
            details: Some(StdErrDetails::Str(msg.into())),
        })
    }

    /// multiple sub errors
    pub fn me<R>(details: Vec<StdErr>) -> Result<R, Self> {
        Err(StdErr {
            code: StdErrCode::Multiple.code(),
            details: Some(StdErrDetails::Sub(details)),
        })
    }
}
