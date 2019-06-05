use std::{env, fs};
use serde::{Serialize, Deserialize};
use config::{Config, ConfigError, File};
use dirs;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub extensions: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            extensions: Default::default(),
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let settings_path = env::var("REBUS_CONFIG_PATH")
            .map(Into::into)
            .unwrap_or_else(|_| {
                let dir = env::var("REBUS_CONFIG_DIR")
                    .map(Into::into)
                    .unwrap_or_else(|_| dirs::home_dir()
                        .map(|d| d.join(concat!(".", env!("CARGO_PKG_NAME"))))
                        .unwrap_or_else(|| "".into()));
                let file = env::var("REBUS_SETTINGS_FILENAME")
                    .unwrap_or_else(|_| "Settings.toml".to_string());

                if !dir.exists() {
                    fs::create_dir_all(dir.as_path())
                        .expect(&format!("Can't create config dir `{}`", dir.display()));
                }

                dir.join(file)
            });

        let mut config = Config::new();
        config.merge(Config::try_from(&Settings::default())?)?;
        config.merge(File::from(settings_path).required(false))?;

        config.try_into()
    }
}