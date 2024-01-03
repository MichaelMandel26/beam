use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::{config::Config, utils::profiles::Profiles};

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct Profile {
    #[serde(skip)]
    pub name: String,
    pub default: bool,
    pub host_pattern: Option<String>,
    pub priority: Option<i32>,
    #[serde(flatten)]
    pub config: Config,
}

impl Profile {
    pub fn new(
        name: String,
        default: bool,
        host_pattern: Option<String>,
        config: Option<Config>,
    ) -> Profile {
        Profile {
            name,
            default,
            host_pattern,
            priority: None,
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

impl From<Vec<Profile>> for Profiles {
    fn from(profiles: Vec<Profile>) -> Self {
        let profile_map = profiles
            .into_iter()
            .map(|profile| (profile.name.to_owned(), profile))
            .collect();
        Profiles {
            profiles: profile_map,
        }
    }
}
