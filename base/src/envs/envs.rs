use crate::{EnvConfig, EnvFile, ErrorCode, OsEnv, StdErrCode, StdR, Values, VoidR};
use config::{Config, File, FileFormat};
use std::path::Path;
use std::sync::{OnceLock, RwLock};

static ENV_CONFIG: OnceLock<RwLock<EnvConfig>> = OnceLock::new();

pub struct Envs;

impl Envs {
    fn os_env() -> OsEnv {
        OsEnv::default()
    }

    /// only once, otherwise raise error
    fn set(config: Config) -> VoidR {
        if ENV_CONFIG
            .set(RwLock::new(EnvConfig::with(config)))
            .is_err()
        {
            StdErrCode::EnvInit.msg("Failed to initialize environment variables again.")
        } else {
            Ok(())
        }
    }

    fn default_config() -> StdR<Config> {
        Config::builder()
            .add_source(File::new("./.env", EnvFile).required(false))
            .add_source(Self::os_env())
            .build()
            .or_else(|e| StdErrCode::EnvInit.msg(e.to_string()))
    }

    pub fn init() -> VoidR {
        Self::set(Self::default_config()?)
    }

    pub fn with_files(files: Vec<String>) -> VoidR {
        if files.len() == 0 {
            return Self::init();
        }

        let mut builder = Config::builder();
        if files.len() == 0 {
            builder = builder.add_source(File::new("./.env", EnvFile).required(false));
        } else {
            for file in files {
                let path = Path::new(&file);
                let ext = if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    Some(ext)
                } else if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    if name.starts_with(".") {
                        Some(name[1..name.len()].as_ref())
                    } else {
                        None
                    }
                } else {
                    None
                };
                builder = match ext {
                    Some("env") => builder.add_source(File::new(file.as_str(), EnvFile)),
                    Some("toml") => builder.add_source(File::new(file.as_str(), FileFormat::Toml)),
                    Some("json") => builder.add_source(File::new(file.as_str(), FileFormat::Json)),
                    Some("json5") => {
                        builder.add_source(File::new(file.as_str(), FileFormat::Json5))
                    }
                    Some("yaml") | Some("yml") => {
                        builder.add_source(File::new(file.as_str(), FileFormat::Yaml))
                    }
                    Some("ini") => builder.add_source(File::new(file.as_str(), FileFormat::Ini)),
                    Some("ron") => builder.add_source(File::new(file.as_str(), FileFormat::Ron)),
                    _ => {
                        return StdErrCode::EnvFileFormatNotSupported
                            .msg(format!("Env file[{}] not supported yet.", file));
                    }
                };
            }
        }

        let config = builder
            .add_source(Self::os_env())
            .build()
            .or_else(|e| StdErrCode::EnvInit.msg(e.to_string()))?;

        Envs::set(config)
    }
}

impl Envs {
    fn env_config() -> &'static RwLock<EnvConfig> {
        ENV_CONFIG.get_or_init(|| match Envs::default_config() {
            Ok(config) => RwLock::new(EnvConfig::with(config)),
            Err(e) => panic!(
                "Failed to initialize environment variables, caused by {}",
                e
            ),
        })
    }

    pub fn get_str(key: &str) -> Option<String> {
        Self::env_config().get_str(key)
    }
}

#[cfg(test)]
mod tests {
    use crate::Envs;
    use std::env::{remove_var, set_var};

    #[test]
    fn test_priority_env_vs_file() {
        unsafe {
            set_var("TEST_KEY", "test value");
        }

        Envs::with_files(vec!["test/.env".to_string()]).expect("Failed to init environment");
        assert_eq!(Envs::get_str("TEST_KEY").unwrap(), "test value");

        unsafe {
            remove_var("TEST_KEY");
        }
    }

    #[test]
    fn test_priority_files() {
        Envs::with_files(vec!["test/.env".to_string(), "test/2.env".to_string()])
            .expect("Failed to init environment");
        assert_eq!(Envs::get_str("TEST_KEY").unwrap(), "test value");
    }

    #[test]
    fn test_json() {
        Envs::with_files(vec!["test/test.json".to_string()]).expect("Failed to init environment");
        assert_eq!(Envs::get_str("test.key").unwrap(), "test value json");
    }
}
