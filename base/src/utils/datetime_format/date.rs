use crate::{
    DateTimeFormatterBase, DateTimeFormatterSupport, EnvConfig, ErrorCode, StdErrCode, StdR, VoidR,
};
use chrono::NaiveDate;
use std::collections::HashMap;
use std::sync::OnceLock;

static DEFAULT_DATE_FORMATS: OnceLock<HashMap<usize, Vec<DateTimeFormatterSupport>>> =
    OnceLock::new();

pub struct DateFormatter;

impl DateTimeFormatterBase<NaiveDate> for DateFormatter {
    fn cache(formats: HashMap<usize, Vec<DateTimeFormatterSupport>>) -> VoidR {
        DEFAULT_DATE_FORMATS
            .set(formats)
            .or_else(|_| StdErrCode::EnvInit.msg("Failed to initialize date formatter."))
    }

    fn default_formats() -> Vec<String> {
        vec![
            // 8 digits
            "%Y%m%d", "%d%m%Y", "%m%d%Y",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    fn formats_from_env(envs: &EnvConfig) -> StdR<Option<Vec<String>>> {
        envs.get_string_vec("DATE_FORMATS")
    }

    fn get_formats(len: &usize) -> Option<&Vec<DateTimeFormatterSupport>> {
        DEFAULT_DATE_FORMATS
            .get_or_init(Self::init_default)
            .get(&len)
    }

    fn try_parse(valid_part: &String, support: &DateTimeFormatterSupport) -> Option<NaiveDate> {
        if let Ok(date) = NaiveDate::parse_from_str(valid_part.as_str(), &support.format) {
            Some(date)
        } else {
            None
        }
    }

    #[track_caller]
    fn format_not_found<R>(str: &String) -> StdR<R> {
        StdErrCode::DateParse.msg(format!(
            "No suitable format for parsing the given string[{}] into a date.",
            str
        ))
    }

    #[track_caller]
    fn parse_failed<R>(str: &String) -> StdR<R> {
        StdErrCode::DateParse.msg(format!(
            "The given string[{}] cannot be parsed into a date.",
            str
        ))
    }
}
