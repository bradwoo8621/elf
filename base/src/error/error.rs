use crate::{ErrorCode, StdErrCode, StdR};
use serde::Serialize;
use std::backtrace::{Backtrace, BacktraceStatus};
use std::fmt::{Display, Formatter};
use std::panic::Location;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::Relaxed;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum StdErrDetail {
    Str(String),
    Sub(Vec<StdErr>),
}

impl Display for StdErrDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Str(s) => {
                write!(f, "{}", s)
            }
            Self::Sub(vec) => {
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
    code: &'static str,
    details: Option<StdErrDetail>,

    // location
    filename: String,
    line: u32,
    column: u32,
}

impl Display for StdErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StdErr[code={}, details={}] at [file={}, line={}, column={}]",
            self.code,
            self.details
                .as_ref()
                .map(|d| format!("{}", d))
                .unwrap_or(String::new()),
            self.filename,
            self.line,
            self.column
        )
    }
}

/// - print backtrace if environment variables [RUST_BACKTRACE] or [RUST_LIB_BACKTRACE] turns on.
/// - always with caller location
impl StdErr {
    //noinspection RsUnstableItemUsage
    /// copy from [Backtrace::enabled]
    fn backtrace_enabled() -> bool {
        // Cache the result of reading the environment variables to make
        // backtrace captures speedy, because otherwise reading environment
        // variables every time can be somewhat slow.
        static ENABLED: AtomicU8 = AtomicU8::new(0);
        match ENABLED.load(Relaxed) {
            0 => {}
            1 => return false,
            _ => return true,
        }
        // do capture, and check status
        let enabled = Backtrace::capture().status() == BacktraceStatus::Captured;
        // println!("RUST_BACKTRACE={:?}, {}", std::env::var("RUST_BACKTRACE"), enabled);
        ENABLED.store(enabled as u8 + 1, Relaxed);
        enabled
    }

    fn print_backtrace() {
        if !Self::backtrace_enabled() {
            return;
        }

        let backtrace = Backtrace::capture();
        match backtrace.status() {
            BacktraceStatus::Captured => {
                println!("{:#?}", backtrace);
            }
            _ => {}
        }
    }

    #[track_caller]
    pub fn create(code: &'static str, details: Option<StdErrDetail>, caller: &Location) -> Self {
        Self::print_backtrace();

        Self {
            code,
            details,
            filename: caller.file().to_string(),
            line: caller.line(),
            column: caller.column(),
        }
    }

    #[track_caller]
    pub fn of<R, M>(code: &'static str, msg: M) -> Result<R, Self>
    where
        M: Into<String>,
    {
        Self::of_with_location(code, msg, Location::caller())
    }

    pub fn of_with_location<R, M>(
        code: &'static str,
        msg: M,
        location: &Location,
    ) -> Result<R, Self>
    where
        M: Into<String>,
    {
        Self::print_backtrace();

        Err(Self {
            code,
            details: Some(StdErrDetail::Str(msg.into())),
            filename: location.file().to_string(),
            line: location.line(),
            column: location.column(),
        })
    }

    #[track_caller]
    pub fn code_only<R>(code: &'static str) -> Result<R, Self> {
        Self::code_only_with_location(code, Location::caller())
    }

    pub fn code_only_with_location<R>(code: &'static str, location: &Location) -> Result<R, Self> {
        Self::print_backtrace();

        Err(Self {
            code,
            details: None,
            filename: location.file().to_string(),
            line: location.line(),
            column: location.column(),
        })
    }

    /// message only
    #[track_caller]
    pub fn unknown<R, M>(msg: M) -> Result<R, Self>
    where
        M: Into<String>,
    {
        Self::unknown_with_location(msg, Location::caller())
    }

    /// message only
    pub fn unknown_with_location<R, M>(msg: M, location: &Location) -> Result<R, Self>
    where
        M: Into<String>,
    {
        Self::print_backtrace();

        Err(Self {
            code: StdErrCode::Unknown.code(),
            details: Some(StdErrDetail::Str(msg.into())),
            filename: location.file().to_string(),
            line: location.line(),
            column: location.column(),
        })
    }

    #[track_caller]
    pub fn accumulate<R>(details: Vec<StdErr>) -> StdR<R> {
        Self::accumulate_with_location(details, Location::caller())
    }

    pub fn accumulate_with_location<R>(details: Vec<StdErr>, location: &Location) -> StdR<R> {
        Self::print_backtrace();

        Err(Self {
            code: StdErrCode::Multiple.code(),
            details: Some(StdErrDetail::Sub(details)),
            filename: location.file().to_string(),
            line: location.line(),
            column: location.column(),
        })
    }
}
