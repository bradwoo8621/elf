use crate::{
    DateFormatter, DateTimeFormatter, DateTimeFormatterBase, DateTimeFormatterSupport, EnvConfig,
    ErrorCode, FullDateTimeFormatter, StdErrCode, StdR, VoidR,
};
use chrono::{NaiveDate, NaiveDateTime};
use std::collections::HashMap;
use std::sync::OnceLock;

static DEFAULT_LOOSE_DATE_FORMATS: OnceLock<HashMap<usize, Vec<DateTimeFormatterSupport>>> =
    OnceLock::new();

/// Loose parsing of date (and/or time) formats, covering all formats of date, datetime, and full datetime.
pub struct LooseDateFormatter;

impl LooseDateFormatter {
    pub fn default_formats() -> Vec<String> {
        let mut formats = DateTimeFormatter::default_formats();
        formats.append(&mut DateTimeFormatter::default_formats());
        formats.append(&mut FullDateTimeFormatter::default_formats());
        formats
    }

    /// init default formats
    fn init_default() -> HashMap<usize, Vec<DateTimeFormatterSupport>> {
        DateTimeFormatterSupport::build_map(Self::default_formats())
    }

    fn formats_of<FromEnv, FromDefault>(
        envs: &EnvConfig,
        formats_from_env: FromEnv,
        default_formats: FromDefault,
    ) -> StdR<Vec<String>>
    where
        FromEnv: FnOnce(&EnvConfig) -> StdR<Option<Vec<String>>>,
        FromDefault: FnOnce() -> Vec<String>,
    {
        let env_formats = if let Some(formats) = formats_from_env(envs)? {
            if formats.len() == 0 {
                None
            } else {
                Some(formats)
            }
        } else {
            None
        };
        if let Some(formats) = env_formats {
            Ok(formats)
        } else {
            Ok(default_formats())
        }
    }

    pub fn formats(envs: &EnvConfig) -> StdR<Vec<String>> {
        let mut formats = Self::formats_of(
            envs,
            DateFormatter::formats_from_env,
            DateTimeFormatter::default_formats,
        )?;
        formats.append(&mut Self::formats_of(
            envs,
            DateTimeFormatter::formats_from_env,
            DateTimeFormatter::default_formats,
        )?);
        formats.append(&mut Self::formats_of(
            envs,
            FullDateTimeFormatter::formats_from_env,
            FullDateTimeFormatter::default_formats,
        )?);

        Ok(formats)
    }

    pub fn init(envs: &EnvConfig) -> VoidR {
        DEFAULT_LOOSE_DATE_FORMATS
            .set(DateTimeFormatterSupport::build_map(Self::formats(envs)?))
            .or_else(|_| StdErrCode::EnvInit.msg("Failed to initialize loose date formatter."))
    }

    fn get_formats(len: &usize) -> Option<&Vec<DateTimeFormatterSupport>> {
        DEFAULT_LOOSE_DATE_FORMATS
            .get_or_init(Self::init_default)
            .get(&len)
    }

    fn format_not_found<R>(str: &String, target_type: &str) -> StdR<R> {
        StdErrCode::TimeParse.msg(format!(
            "No suitable format for parsing the given string[{}] into a {}.",
            str, target_type
        ))
    }

    fn parse_failed<R>(str: &String, target_type: &str) -> StdR<R> {
        StdErrCode::TimeParse.msg(format!(
            "The given string[{}] cannot be parsed into a {}.",
            str, target_type
        ))
    }

    fn parse<T, TryParse>(str: &String, target_type: &str, try_parse: TryParse) -> StdR<T>
    where
        TryParse: Fn(&String, &DateTimeFormatterSupport) -> Option<T>,
    {
        let (valid_part, len) = DateTimeFormatterSupport::valid_part(str);
        if let Some(supports) = Self::get_formats(&len) {
            if supports.len() == 0 {
                Self::format_not_found(str, target_type)
            } else {
                for support in supports {
                    if let Some(time) = try_parse(&valid_part, support) {
                        return Ok(time);
                    }
                }
                Self::parse_failed(str, target_type)
            }
        } else {
            Self::format_not_found(str, target_type)
        }
    }

    pub fn parse_date(str: &String) -> StdR<NaiveDate> {
        Self::parse(str, "date", DateFormatter::try_parse)
    }

    pub fn parse_datetime(str: &String) -> StdR<NaiveDateTime> {
        Self::parse(str, "datetime", DateTimeFormatter::try_parse)
    }

    pub fn parse_full_datetime(str: &String) -> StdR<NaiveDateTime> {
        Self::parse(str, "datetime", FullDateTimeFormatter::try_parse)
    }
}
