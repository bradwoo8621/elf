use crate::{DateTimeFormatterSupport, EnvConfig, StdR, VoidR};
use std::collections::HashMap;

// When formatting date and time, it attempts to perform the formatting
// using the pure numeric parts and plus signs (for time zones) in the string.
// Therefore, only formats with an exact length match will be applied in the formatting attempt.
pub trait DateTimeFormatterBase<T> {
    /// set the given formats into cache
    /// only one call is allowed, any subsequent calls will raise error.
    fn cache(formats: HashMap<usize, Vec<DateTimeFormatterSupport>>) -> VoidR;

    /// get default formats, returns empty vec when there is no default format
    fn default_formats() -> Vec<String>;

    /// init default formats
    fn init_default() -> HashMap<usize, Vec<DateTimeFormatterSupport>> {
        DateTimeFormatterSupport::build_map(Self::default_formats())
    }

    /// get formats from given environment
    fn formats_from_env(envs: &EnvConfig) -> StdR<Option<Vec<String>>>;

    /// init by given environment, or use default when there is no configuration
    fn init(envs: &EnvConfig) -> VoidR {
        let env_formats = if let Some(formats) = Self::formats_from_env(envs)? {
            if formats.len() == 0 {
                None
            } else {
                Some(formats)
            }
        } else {
            None
        };
        let formats = if let Some(formats) = env_formats {
            formats
        } else {
            Self::default_formats()
        };
        Self::cache(DateTimeFormatterSupport::build_map(formats))
    }

    fn get_formats(len: &usize) -> Option<&Vec<DateTimeFormatterSupport>>;

    fn try_parse(valid_part: &String, support: &DateTimeFormatterSupport) -> Option<T>;

    fn format_not_found<R>(str: &String) -> StdR<R>;

    fn parse_failed<R>(str: &String) -> StdR<R>;

    fn parse(str: &String) -> StdR<T> {
        let (valid_part, len) = DateTimeFormatterSupport::valid_part(str);
        if let Some(supports) = Self::get_formats(&len) {
            if supports.len() == 0 {
                Self::format_not_found(str)
            } else {
                for support in supports {
                    if let Some(time) = Self::try_parse(&valid_part, support) {
                        return Ok(time);
                    }
                }
                Self::parse_failed(str)
            }
        } else {
            Self::format_not_found(str)
        }
    }
}
