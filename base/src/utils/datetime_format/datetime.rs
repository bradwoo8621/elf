use crate::{
    DateTimeFormatterBase, DateTimeFormatterSupport, EnvConfig, ErrorCode, LooseDateTimeParser,
    StdErrCode, StdR, VoidR,
};
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::sync::OnceLock;

static DEFAULT_DATETIME_FORMATS: OnceLock<HashMap<usize, Vec<DateTimeFormatterSupport>>> =
    OnceLock::new();

pub struct DateTimeFormatter;

impl DateTimeFormatterBase<NaiveDateTime> for DateTimeFormatter {
    fn cache(formats: HashMap<usize, Vec<DateTimeFormatterSupport>>) -> VoidR {
        DEFAULT_DATETIME_FORMATS
            .set(formats)
            .or_else(|_| StdErrCode::EnvInit.msg("Failed to initialize datetime formatter."))
    }

    fn default_formats() -> Vec<String> {
        vec![
            // 14 digits
            "%Y%m%d%H%M%S",
            "%d%m%Y%H%M%S",
            "%m%d%Y%H%M%S",
            // 12 digits
            "%Y%m%d%H%M",
            "%d%m%Y%H%M",
            "%m%d%Y%H%M",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    fn formats_from_env(envs: &EnvConfig) -> StdR<Option<Vec<String>>> {
        envs.get_string_vec("DATETIME_FORMATS")
    }

    fn get_formats(len: &usize) -> Option<&Vec<DateTimeFormatterSupport>> {
        DEFAULT_DATETIME_FORMATS
            .get_or_init(Self::init_default)
            .get(&len)
    }

    // noinspection DuplicatedCode
    fn try_parse(valid_part: &String, support: &DateTimeFormatterSupport) -> Option<NaiveDateTime> {
        if let Ok(datetime) = LooseDateTimeParser::parse(valid_part, &support.format) {
            Some(datetime)
        } else {
            None
        }
    }

    #[track_caller]
    fn format_not_found<R>(str: &String) -> StdR<R> {
        StdErrCode::DateTimeParse.msg(format!(
            "No suitable format for parsing the given string[{}] into a datetime.",
            str
        ))
    }

    #[track_caller]
    fn parse_failed<R>(str: &String) -> StdR<R> {
        StdErrCode::DateTimeParse.msg(format!(
            "The given string[{}] cannot be parsed into a datetime.",
            str
        ))
    }
}
