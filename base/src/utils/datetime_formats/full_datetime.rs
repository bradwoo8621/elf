use crate::{
    DateTimeFormatterBase, DateTimeFormatterSupport, EnvConfig, ErrorCode, LooseDateTimeParser,
    StdErrCode, StdR, VoidR,
};
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::sync::OnceLock;

static DEFAULT_FULL_DATETIME_FORMATS: OnceLock<HashMap<usize, Vec<DateTimeFormatterSupport>>> =
    OnceLock::new();

pub struct FullDateTimeFormatter;

impl DateTimeFormatterBase<NaiveDateTime> for FullDateTimeFormatter {
    fn cache(formats: HashMap<usize, Vec<DateTimeFormatterSupport>>) -> VoidR {
        DEFAULT_FULL_DATETIME_FORMATS
            .set(formats)
            .or_else(|_| StdErrCode::EnvInit.msg("Failed to initialize full datetime formatter."))
    }

    fn default_formats() -> Vec<String> {
        vec![
            // 14 or more digits
            "%Y%m%d%H%M%S%f",
            "%d%m%Y%H%M%S%f",
            "%m%d%Y%H%M%S%f",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    fn formats_from_env(envs: &EnvConfig) -> StdR<Option<Vec<String>>> {
        envs.get_string_vec("FULL_DATETIME_FORMATS")
    }

    fn get_formats(len: &usize) -> Option<&Vec<DateTimeFormatterSupport>> {
        DEFAULT_FULL_DATETIME_FORMATS
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

    fn format_not_found<R>(str: &String) -> StdR<R> {
        StdErrCode::FullDateTimeParse.msg(format!(
            "No suitable format for parsing the given string[{}] into a full datetime.",
            str
        ))
    }

    fn parse_failed<R>(str: &String) -> StdR<R> {
        StdErrCode::FullDateTimeParse.msg(format!(
            "The given string[{}] cannot be parsed into a full datetime.",
            str
        ))
    }
}
