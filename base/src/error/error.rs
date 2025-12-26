use crate::{ErrorCode, StdErrCode, StdR};
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum StdErrDetail {
    Str(String),
    Sub(Vec<StdErr>),
}

impl Display for StdErrDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StdErrDetail::Str(s) => {
                write!(f, "{}", s)
            }
            StdErrDetail::Sub(vec) => {
                write!(
                    f,
                    "{}",
                    vec.iter()
                        .map(|se| format!("{}", se))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
}

/// In theory, errors support an infinite number of levels.
/// However, in normal use, you should try to keep it to two levels.
///
/// Convert other types of exceptions to this exception to enable the use of the `?` syntactic sugar.
#[derive(Serialize, Debug)]
pub struct StdErr {
    /// code must be [XXXX-99999], each module has its own code prefix [XXXX]
    pub code: &'static str,
    pub details: Option<StdErrDetail>,
}

impl Display for StdErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StdErr[code={}, details={}]",
            self.code,
            self.details
                .as_ref()
                .map(|d| format!("{}", d))
                .unwrap_or(String::new()),
        )
    }
}

impl StdErr {
    pub fn of<R, M>(code: &'static str, msg: M) -> Result<R, Self>
    where
        M: Into<String>,
    {
        Err(Self {
            code,
            details: Some(StdErrDetail::Str(msg.into())),
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
            details: Some(StdErrDetail::Str(msg.into())),
        })
    }

    pub fn accumulate<R>(details: Vec<StdErr>) -> StdR<R> {
        Err(Self {
            code: StdErrCode::Multiple.code(),
            details: Some(StdErrDetail::Sub(details)),
        })
    }
}
