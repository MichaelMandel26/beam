use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap as Map, process};

use crate::utils::config::Config;

const BEAM_PROFILES_PATH: &str = ".beam/profiles.toml";

lazy_static! {
    #[derive(Debug, PartialEq, Eq, Default)]
    pub static ref DEFAULT_PROFILE: Profile = match Profiles::get_default() {
        Ok(profile) => profile,
        Err(err) => {
            println!("Error: {}", err);
            process::exit(1);
        }
    };
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Profiles {
    #[serde(rename = "profile")]
    pub profiles: Map<String, Profile>,
}

impl Profiles {
    pub fn write(profile: Profile) -> Result<()> {
        let mut profiles = Profiles::get_profiles()?;

        if profile.default {
            profiles
                .profiles
                .values_mut()
                .for_each(|p| p.default = false);
        }

        let key = profile.name.as_str();
        profiles
            .profiles
            .entry(key.to_string())
            .or_insert_with(Profile::default);
        profiles.profiles.insert(key.to_string(), profile);
        profiles.save()?;
        Ok(())
    }

    fn save(&self) -> Result<()> {
        let profiles_path = home::home_dir().unwrap().join(BEAM_PROFILES_PATH);
        let profiles_str = toml::to_string(&self)?;
        std::fs::write(profiles_path, profiles_str)?;
        Ok(())
    }

    fn get_profiles() -> Result<Profiles> {
        let profiles_path = home::home_dir().unwrap().join(BEAM_PROFILES_PATH);
        let profiles_str = std::fs::read_to_string(profiles_path)
            .context("Error while reading Profiles from profiles.toml")?;
        let mut profiles_map: Profiles = toml::from_str(&profiles_str)?;
        for (key, profile) in profiles_map.profiles.iter_mut() {
            profile.name = key.to_string();
        }

        Ok(profiles_map)
    }

    pub fn get() -> Result<Vec<Profile>> {
        let profiles_map = Profiles::get_profiles()?;
        let profiles = profiles_map
            .profiles
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<Profile>>();
        Profiles::verify_default(&profiles)?;
        Ok(profiles)
    }

    pub fn get_default() -> Result<Profile> {
        let profiles = Profiles::get()?;
        Profiles::verify_default(&profiles)
    }

    pub fn verify_default(profiles: &[Profile]) -> Result<Profile> {
        let default_profiles_count = profiles.iter().filter(|profile| profile.default).count();
        match default_profiles_count {
            0 => Err(anyhow!("No default profile found. Please create a default profile.")),
            1 => Ok(profiles.iter().find(|profile| profile.default).unwrap().clone()),
            _ => Err(anyhow!("Multiple default profiles found. Please fix your profiles.toml")),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Profile {
    #[serde(skip)]
    pub name: String,
    pub default: bool,
    #[serde(flatten)]
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

    pub fn get(name: &str) -> Result<Profile> {
        let profiles = Profiles::get_profiles().unwrap();
        let profile = profiles.profiles.get(name);
        match profile {
            Some(profile) => Ok(profile.to_owned()),
            None => Err(anyhow!("Could not find profile {}", name)),
        }
    }
}
