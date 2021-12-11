use anyhow::Result;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

use crate::utils::config::Config;

const BEAM_PROFILES_PATH: &str = ".beam/profiles.toml";

lazy_static! {
    pub static ref DEFAULT_PROFILE: Profile = Profile::get_default().unwrap();
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub name: String,
    pub default: bool,
    pub config: Config,
}

impl Profile {
    pub fn new(name: String, default: bool, config: Option<Config>) -> Profile {
        Profile {
            name,
            default,
            config: config.unwrap_or_default(),
        }
    }

    //     pub fn write(&self) -> Result<()> {
    //     let config_path = home::home_dir().unwrap().join(BEAM_PATH);
    //     let config_str = toml::to_string(&self)?;
    //     std::fs::write(config_path, config_str)?;
    //     Ok(())
    // }

    pub fn get_default() -> Result<Profile> {
        let profiles_path = home::home_dir().unwrap().join(BEAM_PROFILES_PATH);
        let profiles_str = std::fs::read_to_string(profiles_path)?;
        let profiles: Vec<Profile> = toml::from_str(&profiles_str)?;
        let default_profile = profiles.iter().find(|profile| profile.default);
        if let Some(default_profile) = default_profile {
            Ok(*default_profile)
        } else {
            Err(anyhow::anyhow!("No default profile found"))
        }
    }

}