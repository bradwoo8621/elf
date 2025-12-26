use config::{ConfigError, Map, Source, Value, ValueKind};
use std::env;
use std::ffi::OsString;

/// copy from [config::Environment],
/// no key lowercase, no empty value ignore, no prefix pattern, no try parsing.
/// basically, read from os environment, and returns what it is.
#[derive(Clone, Debug, Default)]
pub struct OsEnv;

impl Source for OsEnv {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<Map<String, Value>, ConfigError> {
        let mut m = Map::new();
        let uri: String = "the environment".into();

        let collector = |(key, value): (OsString, OsString)| {
            let key = match key.into_string() {
                Ok(key) => key,
                // Key is not valid unicode, skip it
                Err(_) => return Ok(()),
            };

            // At this point, we don't know if the key is required or not.
            // Therefore, if the value is not a valid unicode string, we error out.
            let value = value.into_string().map_err(|os_string| {
                ConfigError::Message(format!(
                    "env variable {key:?} contains non-Unicode data: {os_string:?}"
                ))
            })?;

            let value = ValueKind::String(value);

            m.insert(key, Value::new(Some(&uri), value));

            Ok(())
        };

        env::vars_os().try_for_each(collector)?;

        Ok(m)
    }
}
