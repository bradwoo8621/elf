use crate::{ErrorCode, StdErrCode, StdR, StringUtils};
use bigdecimal::BigDecimal;
use config::{Config, ConfigError};
use std::str::FromStr;

pub struct EnvConfig {
    config: Config,
}

impl EnvConfig {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn get_bool(&self, key: &str) -> StdR<Option<bool>> {
        match self.config.get_string(key) {
            Ok(value) => match value.to_ascii_lowercase().as_str() {
                "true" | "t" | "yes" | "y" | "on" | "1" => Ok(Some(true)),
                "false" | "f" | "no" | "n" | "off" | "0" => Ok(Some(false)),
                s => StdErrCode::EnvValueTypeMismatch.msg(format!(
                    "Invalid value[{}={}] from environment, cannot be parsed to boolean.",
                    key, s,
                )),
            },
            Err(ConfigError::NotFound(_)) => Ok(None),
            Err(e) => StdErrCode::EnvValueGet.msg(e.to_string()),
        }
    }

    pub fn get_string(&self, key: &str) -> StdR<Option<String>> {
        match self.config.get_string(key) {
            Ok(s) => Ok(Some(s)),
            Err(ConfigError::NotFound(_)) => Ok(None),
            Err(e) => StdErrCode::EnvValueGet.msg(e.to_string()),
        }
    }

    pub fn get_usize(&self, key: &str) -> StdR<Option<usize>> {
        if let Some(value) = self.get_int(key)? {
            if value < 0 {
                return StdErrCode::EnvValueTypeMismatch.msg(format!(
                    "Environment[{}={}] is not allowed, value cannot be negative.",
                    key, value
                ));
            }
            if value > (usize::MAX as i64) {
                return StdErrCode::EnvValueTypeMismatch.msg(format!(
                    "Environment[{}={}] is not allowed, value cannot be greater than {}.",
                    key,
                    value,
                    usize::MAX
                ));
            }
            Ok(Some(value as usize))
        } else {
            Ok(None)
        }
    }

    pub fn get_int(&self, key: &str) -> StdR<Option<i64>> {
        let value = self.config.get_int(key);
        match value {
            Ok(value) => Ok(Some(value)),
            Err(ConfigError::NotFound(_)) => Ok(None),
            Err(e) => StdErrCode::EnvValueGet.msg(e.to_string()),
        }
    }

    pub fn get_decimal(&self, key: &str) -> StdR<Option<BigDecimal>> {
        match self.config.get_string(key) {
            Ok(value) => {
                BigDecimal::from_str(&value)
                    .or_else(|e| {
                        StdErrCode::EnvValueTypeMismatch.msg(format!(
                            "Invalid value[{}={}] from environment, cannot be parsed to decimal, caused by {}.",
                            key, value, e
                        ))
                    })
                    .map(Some)
            },
            Err(ConfigError::NotFound(_)) => Ok(None),
            Err(e) => StdErrCode::EnvValueGet.msg(e.to_string()),
        }
    }

    pub fn get_string_vec(&self, key: &str) -> StdR<Option<Vec<String>>> {
        match self.config.get_string(key) {
            Ok(value) => {
                if value.is_blank() {
                    StdErrCode::EnvValueTypeMismatch.msg(format!(
                        "Invalid value[{}={}] from environment, cannot be parsed blank string to vec.",
                        key, value
                    ))
                } else {
                    Ok(Some(
                        value.split(',').map(|s| s.trim().to_string()).collect(),
                    ))
                }
            }
            Err(ConfigError::NotFound(_)) => Ok(None),
            Err(e) => StdErrCode::EnvValueGet.msg(e.to_string()),
        }
    }
}
