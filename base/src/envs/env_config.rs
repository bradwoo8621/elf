use crate::{ErrorCode, StdErrCode, StdR, StringUtils};
use bigdecimal::BigDecimal;
use config::Config;
use std::str::FromStr;

pub struct EnvConfig {
    config: Config,
}

impl EnvConfig {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn get_bool(&self, key: &str) -> StdR<Option<bool>> {
        let value = self
            .config
            .get_string(key)
            .or_else(|e| StdErrCode::EnvValueGet.msg(e.to_string()))?;
        match value.to_ascii_lowercase().as_str() {
            "true" | "t" | "yes" | "y" | "on" | "1" => Ok(Some(true)),
            "false" | "f" | "no" | "n" | "off" | "0" => Ok(Some(false)),
            s => StdErrCode::EnvValueTypeMismatch.msg(format!(
                "Invalid value[{}={}] from environment, cannot be parsed to boolean.",
                key, s,
            )),
        }
    }

    pub fn get_string(&self, key: &str) -> StdR<Option<String>> {
        match self.config.get_string(key) {
            Ok(s) => Ok(Some(s)),
            Err(e) => StdErrCode::EnvValueGet.msg(e.to_string()),
        }
    }

    pub fn get_int(&self, key: &str) -> StdR<Option<i64>> {
        self.config
            .get_int(key)
            .or_else(|e| StdErrCode::EnvValueGet.msg(e.to_string()))
            .map(Some)
    }

    pub fn get_decimal(&self, key: &str) -> StdR<Option<BigDecimal>> {
        let value = self
            .config
            .get_string(key)
            .or_else(|e| StdErrCode::EnvValueGet.msg(e.to_string()))?;
        BigDecimal::from_str(&value)
            .or_else(|e| {
                StdErrCode::EnvValueTypeMismatch.msg(format!(
                    "Invalid value[{}={}] from environment, cannot be parsed to decimal, caused by {}.",
                    key, value, e
                ))
            })
            .map(Some)
    }

    pub fn get_string_vec(&self, key: &str) -> StdR<Option<Vec<String>>> {
        let value = self
            .config
            .get_string(key)
            .or_else(|e| StdErrCode::EnvValueGet.msg(e.to_string()))?;
        if value.is_blank() {
            StdErrCode::EnvValueTypeMismatch.msg(format!(
                "Invalid value[{}={}] from environment, cannot be parsed blank string to vec.",
                key, value
            ))
        } else {
            Ok(Some(value.split(',').map(|s| s.to_string()).collect()))
        }
    }
}
