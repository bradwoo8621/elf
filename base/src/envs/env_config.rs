use crate::{ErrorCode, StdErrCode, StdR, StringUtils};
use bigdecimal::BigDecimal;
use config::Config;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::RwLock;

pub enum EnvValue {
    None,
    Bool(bool),
    Str(String),
    Int(i64),
    Decimal(BigDecimal),
    StrVec(Vec<String>),
}

pub struct EnvConfig {
    // origin config variables
    config: Config,
    // variable which is read at least once, and might be cast into other type
    map: HashMap<String, EnvValue>,
}

impl EnvConfig {
    pub fn with(config: Config) -> Self {
        Self {
            config,
            map: HashMap::new(),
        }
    }
}

pub trait Values {
    fn get<V, OnMapValue, FromConfig, ToMapValue>(
        &self,
        key: &str,
        on_map_value: OnMapValue,
        from_config: FromConfig,
        to_map_value: ToMapValue,
    ) -> Option<V>
    where
        OnMapValue: FnOnce(&EnvValue) -> Option<V>,
        FromConfig: FnOnce(&Config, &str) -> StdR<V>,
        ToMapValue: FnOnce(&V) -> EnvValue;

    fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(
            key,
            |value| match value {
                EnvValue::Bool(b) => Some(*b),
                _ => None,
            },
            |config, key| {
                let value = config
                    .get_string(key)
                    .or_else(|e| StdErrCode::EnvValueGet.msg(e.to_string()))?;
                match value.to_ascii_lowercase().as_str() {
                    "true" | "t" | "yes" | "y" | "on" | "1" => Ok(true),
                    "false" | "f" | "no" | "n" | "off" | "0" => Ok(false),
                    s => StdErrCode::EnvValueTypeMismatch.msg(format!(
                        "Invalid value[{}={}] from environment, cannot be parsed to boolean.",
                        key, s,
                    )),
                }
            },
            |value| EnvValue::Bool(*value),
        )
    }

    fn get_bool_or_default(&self, key: &str, default_value: bool) -> bool {
        self.get_bool(key).unwrap_or(default_value)
    }

    fn get_str(&self, key: &str) -> Option<String> {
        self.get(
            key,
            |value| {
                match value {
                    EnvValue::Str(s) => Some(s.clone()),
                    EnvValue::None => None,
                    // type not matched, but return none for now
                    _ => None,
                }
            },
            |config, key| {
                config
                    .get_string(key)
                    .or_else(|e| StdErrCode::EnvValueGet.msg(e.to_string()))
            },
            |value| EnvValue::Str(value.clone()),
        )
    }

    fn get_str_or_default(&self, key: &str, default_value: String) -> String {
        self.get_str(key).unwrap_or(default_value)
    }

    fn get_int(&self, key: &str) -> Option<i64> {
        self.get(
            key,
            |value| match value {
                EnvValue::Int(i) => Some(*i),
                _ => None,
            },
            |config, key| {
                config
                    .get_int(key)
                    .or_else(|e| StdErrCode::EnvValueGet.msg(e.to_string()))
            },
            |value| EnvValue::Int(*value),
        )
    }

    fn get_int_or_default(&self, key: &str, default_value: i64) -> i64 {
        self.get_int(key).unwrap_or(default_value)
    }

    fn get_vec(&self, key: &str) -> Option<Vec<String>> {
        self.get(
            key,
            |value| match value {
                EnvValue::StrVec(d) => Some(d.clone()),
                _ => None,
            },
            |config, key| {
                let value = config
                    .get_string(key)
                    .or_else(|e| StdErrCode::EnvValueGet.msg(e.to_string()))?;
                if value.is_blank() {
                    StdErrCode::EnvValueTypeMismatch.msg(format!(
                        "Invalid value[{}={}] from environment, cannot be parsed blank string to vec.",
                        key, value
                    ))
                } else {
                    Ok(value.split(',').map(|s| s.to_string()).collect())
                }
            },
            |value| EnvValue::StrVec(value.clone()),
        )
    }

    fn get_vec_or_default(&self, key: &str, default_value: Vec<String>) -> Vec<String> {
        self.get_vec(key).unwrap_or(default_value)
    }

    fn get_decimal(&self, key: &str) -> Option<BigDecimal> {
        self.get(
            key,
            |value| match value {
                EnvValue::Decimal(d) => Some(d.clone()),
                _ => None,
            },
            |config, key| {
                let value = config
                    .get_string(key)
                    .or_else(|e| StdErrCode::EnvValueGet.msg(e.to_string()))?;
                BigDecimal::from_str(&value).or_else(|e| {
                    StdErrCode::EnvValueTypeMismatch.msg(format!(
                        "Invalid value[{}={}] from environment, cannot be parsed to decimal, caused by {}.",
                        key, value, e
                    ))
                })
            },
            |value| EnvValue::Decimal(value.clone()),
        )
    }

    fn get_decimal_or_default(&self, key: &str, default_value: BigDecimal) -> BigDecimal {
        self.get_decimal(key).unwrap_or(default_value)
    }
}

impl Values for RwLock<EnvConfig> {
    fn get<V, OnMapValue, FromConfig, ToMapValue>(
        &self,
        key: &str,
        on_map_value: OnMapValue,
        from_config: FromConfig,
        to_map_value: ToMapValue,
    ) -> Option<V>
    where
        OnMapValue: FnOnce(&EnvValue) -> Option<V>,
        FromConfig: FnOnce(&Config, &str) -> StdR<V>,
        ToMapValue: FnOnce(&V) -> EnvValue,
    {
        let value_from_config = {
            // TIP make the read lock release after this block.
            let ec = self.read().unwrap();
            if let Some(value) = ec.map.get(key) {
                return match value {
                    EnvValue::None => None,
                    _ => on_map_value(value),
                }
            } else {
                from_config(&ec.config, key)
            }
        };

        if let Ok(value) = value_from_config {
            self.write()
                .unwrap()
                .map
                .insert(key.to_string(), to_map_value(&value));
            Some(value)
        } else {
            // not ok when get from config,
            self.write()
                .unwrap()
                .map
                .insert(key.to_string(), EnvValue::None);
            None
        }
    }
}
