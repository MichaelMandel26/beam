use std::io::Read;

use crate::ConfigOpts;
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub username: Option<String>,
    pub cache_ttl: Option<u64>,
}

const BEAM_PATH: &str = ".beam/config.toml";

pub fn config(cfg: ConfigOpts) -> Result<()> {
    ensure!(cfg != Default::default(), "No config options specified");

    let mut config = get_config()?.unwrap_or_default();

    if let Some(username) = cfg.username {
        config.username = Some(username);
    }

    if let Some(cache_ttl) = cfg.cache_ttl {
        config.cache_ttl = Some(cache_ttl);
    }

    write_config(config)?;

    println!("Config successfully updated");

    Ok(())
}

pub fn get_config() -> Result<Option<Config>> {
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


fn write_config(config: Config) -> Result<()> {
    let config_path = home::home_dir().unwrap().join(BEAM_PATH);
    let config_str = toml::to_string(&config)?;
    std::fs::write(config_path, config_str)?;    
    Ok(())
}
