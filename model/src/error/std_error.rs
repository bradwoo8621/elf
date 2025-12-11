use serde::Serialize;

pub trait StdErrorCode {
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
            details: Some(StdErrDetails::Str(msg.into())),
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
    StrEnumParse,
    VariableFunctionParse,
    /// with multiple sub errors
    Multiple,
    Unknown,
}

impl StdErrorCode for StdErrCode {
    fn code(&self) -> &'static str {
        match self {
            StdErrCode::DecimalParse => "STDE-00001",
            StdErrCode::StrEnumParse => "STDE-00002",
            StdErrCode::VariableFunctionParse => "STDE-00003",

            StdErrCode::Multiple => "STDE-99998",
            StdErrCode::Unknown => "STDE-99999",
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum StdErrDetails {
    Str(String),
    Sub(Vec<StdErr>),
}

/// In theory, errors support an infinite number of levels.
/// However, in normal use, you should try to keep it to two levels.
///
/// Convert other types of exceptions to this exception to enable the use of the `?` syntactic sugar.
#[derive(Serialize, Debug)]
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
        Err(Self {
            code,
            details: Some(StdErrDetails::Str(msg.into())),
        })
    }

    pub fn code_only<R>(code: &'static str) -> Result<R, Self> {
        Err(Self {
            code,
            details: None,
        })
    }

    /// message only
    pub fn unknown<R, M>(msg: M) -> Result<R, Self>
    where
        M: Into<String>,
    {
        Err(Self {
            code: StdErrCode::Unknown.code(),
            details: Some(StdErrDetails::Str(msg.into())),
        })
    }

    pub fn accumulate<R>(details: Vec<StdErr>) -> StdR<R> {
        Err(Self {
            code: StdErrCode::Multiple.code(),
            details: Some(StdErrDetails::Sub(details)),
        })
    }
}

pub type StdR<T> = Result<T, StdErr>;
pub type VoidR = StdR<()>;

pub trait VoidResultHelper {
    fn collect(self, result: VoidR) -> Self;
    fn accumulate(self) -> VoidR;
}

impl VoidResultHelper for Vec<StdErr> {
    fn collect(mut self, result: VoidR) -> Self {
        if let Err(e) = result {
            self.push(e);
        }
        self
    }

    fn accumulate(mut self) -> VoidR {
        match self.len() {
            0 => Ok(()),
            1 => Err(self.remove(0)),
            _ => StdErr::accumulate(self),
        }
    }
}
