use crate::{
    DateTimeFormatterBase, DateTimeFormatterSupport, EnvConfig, ErrorCode, StdErrCode, StdR, VoidR,
};
use chrono::NaiveTime;
use std::collections::HashMap;
use std::sync::OnceLock;

static DEFAULT_TIME_FORMATS: OnceLock<HashMap<usize, Vec<DateTimeFormatterSupport>>> =
    OnceLock::new();

pub struct TimeFormatter;

impl DateTimeFormatterBase<NaiveTime> for TimeFormatter {
    fn cache(formats: HashMap<usize, Vec<DateTimeFormatterSupport>>) -> VoidR {
        DEFAULT_TIME_FORMATS
            .set(formats)
            .or_else(|_| StdErrCode::EnvInit.msg("Failed to initialize time formatter."))
    }

    fn default_formats() -> Vec<String> {
        vec![
            "%H%M%S", // 6 digits
            "%H%M",   // 4 digits
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    fn formats_from_env(envs: &EnvConfig) -> StdR<Option<Vec<String>>> {
        envs.get_string_vec("TIME_FORMATS")
    }

    fn get_formats(len: &usize) -> Option<&Vec<DateTimeFormatterSupport>> {
        DEFAULT_TIME_FORMATS
            .get_or_init(Self::init_default)
            .get(&len)
    }

    fn try_parse(valid_part: &String, support: &DateTimeFormatterSupport) -> Option<NaiveTime> {
        if let Ok(time) = NaiveTime::parse_from_str(valid_part.as_str(), &support.format) {
            Some(time)
        } else {
            None
        }
    }

    fn format_not_found<R>(str: &String) -> StdR<R> {
        StdErrCode::TimeParse.msg(format!(
            "No suitable format for parsing the given string[{}] into a time.",
            str
        ))
    }

    fn parse_failed<R>(str: &String) -> StdR<R> {
        StdErrCode::TimeParse.msg(format!(
            "The given string[{}] cannot be parsed into a time.",
            str
        ))
    }
}
