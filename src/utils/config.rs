use std::{
    fmt::{Display, Formatter},
    io::Read,
};

use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

const BEAM_PATH: &str = ".beam/config.toml";

lazy_static! {
    pub static ref CONFIG: Config = Config::get().unwrap_or_default().unwrap_or_default();
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub username: Option<String>,
    pub proxy: Option<String>,
    pub auth: Option<String>,
    pub cache_ttl: Option<u64>,
    pub label_whitelist: Option<Vec<String>>,
}

impl Config {
    pub fn get() -> Result<Option<Config>> {
        let config_path = home::home_dir().unwrap().join(BEAM_PATH);

        if !config_path.exists() {
            return Ok(None);
        }

        let mut config_file = std::fs::File::open(config_path)?;
        let mut config_str = String::new();
        config_file.read_to_string(&mut config_str)?;

        let config: Config = toml::from_str(&config_str)?;

        Ok(Some(config))
    }

    pub fn write(&self) -> Result<()> {
        let config_path = home::home_dir().unwrap().join(BEAM_PATH);
        let config_str = toml::to_string(&self)?;
        std::fs::write(config_path, config_str)?;
        Ok(())
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", toml::to_string(&self).unwrap())
    }
}
