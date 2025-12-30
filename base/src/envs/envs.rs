use crate::{EnvConfig, EnvFile, ErrorCode, OsEnv, StdErrCode, StdR};
use config::{Config, File, FileFormat};
use std::path::Path;

pub struct Envs;

/// Load environment variables, including the specified environment variable file
/// (if specified; otherwise, load the [.env] file in the execution directory).
/// Environment variables are not retained.
///
/// Therefore, the environment variables loaded at different times may vary.
/// Typically, environment variables are loaded when the system starts and are not reloaded afterward.
/// If the loaded environment variables need to be used continuously,
/// consume the returned [Config] to store them elsewhere and maintain reasonable references.
impl Envs {
    fn os_env() -> OsEnv {
        OsEnv::default()
    }

    pub fn init() -> StdR<EnvConfig> {
        let config = Config::builder()
            .add_source(File::new("./.env", EnvFile).required(false))
            .add_source(Self::os_env())
            .build()
            .or_else(|e| StdErrCode::EnvInit.msg(e.to_string()))?;

        Ok(EnvConfig::new(config))
    }

    pub fn with_files(files: Vec<String>) -> StdR<EnvConfig> {
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

        Ok(EnvConfig::new(config))
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

        let config =
            Envs::with_files(vec!["test/.env".to_string()]).expect("Failed to init environment");
        assert_eq!(
            config.get_string("TEST_KEY").unwrap().unwrap().as_str(),
            "test value"
        );

        unsafe {
            remove_var("TEST_KEY");
        }
    }

    #[test]
    fn test_priority_files() {
        let config = Envs::with_files(vec!["test/.env".to_string(), "test/2.env".to_string()])
            .expect("Failed to init environment");
        assert_eq!(
            config.get_string("TEST_KEY").unwrap().unwrap().as_str(),
            "test value"
        );
    }

    #[test]
    fn test_json() {
        let config = Envs::with_files(vec!["test/test.json".to_string()])
            .expect("Failed to init environment");
        assert_eq!(
            config.get_string("test.key").unwrap().unwrap().as_str(),
            "test value json"
        );
    }
}
