use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap as Map, fs};

use crate::{command, utils::profile::Profile};

const BEAM_PROFILES_PATH: &str = ".beam/profiles.toml";

lazy_static! {
    #[derive(Debug, PartialEq, Eq, Default)]
    pub static ref DEFAULT_PROFILE: Profile = match Profiles::get_default() {
        Ok(profile) => profile,
        Err(err) => {
            println!("{}", err);
            command::configure::default::Default::run().unwrap();
            Profiles::get_default().unwrap()
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
        let mut profiles = Profiles::get_profiles().unwrap_or_default();

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

    pub fn save(&self) -> Result<()> {
        let profiles_path = home::home_dir().unwrap().join(BEAM_PROFILES_PATH);
        if !profiles_path.exists() {
            fs::create_dir_all(profiles_path.parent().unwrap())?;
            fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(&profiles_path)?;
        }
        let profiles_str = if self.profiles.is_empty() {
            "".to_string()
        } else {
            toml::to_string(&self)?
        };
        std::fs::write(profiles_path, profiles_str)?;
        Ok(())
    }

    pub fn get_profiles() -> Result<Profiles> {
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
        let profiles_map = Profiles::get_profiles();
        match profiles_map {
            Ok(profiles_map) => {
                let profiles = profiles_map
                    .profiles
                    .into_iter()
                    .map(|(_, v)| v)
                    .collect::<Vec<Profile>>();
                Profiles::verify_profiles_integrity(&profiles)?;
                Ok(profiles)
            }
            Err(_) => Ok(vec![]),
        }
    }

    pub fn get_default() -> Result<Profile> {
        let profiles = Profiles::get()?;
        Profiles::verify_profiles_integrity(&profiles)
    }

    pub fn verify_profiles_integrity(profiles: &[Profile]) -> Result<Profile> {
        if profiles.is_empty() {
            return Err(anyhow!("No profiles found"));
        }

        let default_profiles_count = profiles.iter().filter(|profile| profile.default).count();
        match default_profiles_count {
            0 => Err(anyhow!(
                "No default profile found. Please create a default profile."
            )),
            1 => Ok(profiles
                .iter()
                .find(|profile| profile.default)
                .unwrap()
                .clone()),
            _ => Err(anyhow!(
                "Multiple default profiles found. Please fix your profiles.toml"
            )),
        }
    }

    pub fn get_names(profiles: &[Profile]) -> Result<Vec<String>> {
        Ok(profiles
            .iter()
            .map(|p| {
                if p.default {
                    format!("{} (default)", p.name)
                } else {
                    p.name.clone()
                }
            })
            .collect::<Vec<_>>())
    }

    pub fn get_matching(hostname: &str, profiles: Vec<Profile>) -> Result<Option<Profile>> {
        for profile in profiles {
            if profile.host_pattern.is_some() {
                let regex = Regex::new(profile.host_pattern.as_ref().unwrap())?;
                if regex.is_match(hostname) {
                    return Ok(Some(profile));
                }
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::config::Config;

    use super::*;

    #[test]
    fn test_verify_profiles_integrity() {
        let valid_profiles = [
            Profile {
                name: "test".to_owned(),
                config: Config {
                    username: Some("test".to_owned()),
                    auth: Some("test".to_owned()),
                    proxy: Some("test".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test".to_owned()]),
                },
                default: true,
                host_pattern: None,
            },
            Profile {
                name: "test2".to_owned(),
                config: Config {
                    username: Some("test2".to_owned()),
                    auth: Some("test2".to_owned()),
                    proxy: Some("test2".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test2".to_owned()]),
                },
                default: false,
                host_pattern: None,
            },
        ];
        let invalid_profiles = [
            Profile {
                name: "test".to_owned(),
                config: Config {
                    username: Some("test".to_owned()),
                    auth: Some("test".to_owned()),
                    proxy: Some("test".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test".to_owned()]),
                },
                default: true,
                host_pattern: None,
            },
            Profile {
                name: "test2".to_owned(),
                config: Config {
                    username: Some("test2".to_owned()),
                    auth: Some("test2".to_owned()),
                    proxy: Some("test2".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test2".to_owned()]),
                },
                default: true,
                host_pattern: None,
            },
        ];

        assert!(Profiles::verify_profiles_integrity(&valid_profiles).is_ok());
        assert!(Profiles::verify_profiles_integrity(&invalid_profiles).is_err());
        assert!(Profiles::verify_profiles_integrity(&[]).is_err());
    }

    #[test]
    fn test_get_names() {
        let profiles = [
            Profile {
                name: "test".to_owned(),
                config: Config {
                    username: Some("test".to_owned()),
                    auth: Some("test".to_owned()),
                    proxy: Some("test".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test".to_owned()]),
                },
                default: true,
                host_pattern: None,
            },
            Profile {
                name: "test2".to_owned(),
                config: Config {
                    username: Some("test2".to_owned()),
                    auth: Some("test2".to_owned()),
                    proxy: Some("test2".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test2".to_owned()]),
                },
                default: false,
                host_pattern: None,
            },
        ];
        let expected_names = vec!["test (default)", "test2"];
        assert_eq!(Profiles::get_names(&profiles).unwrap(), expected_names);
        assert_eq!(Profiles::get_names(&[]).unwrap(), Vec::<String>::from([]));
    }

    #[test]
    fn test_get_matching() {
        let hostname = "quality.app.example.com";
        let expected_profile = Profile {
            name: "test".to_owned(),
            config: Config {
                username: Some("test".to_owned()),
                auth: Some("test".to_owned()),
                proxy: Some("test".to_owned()),
                cache_ttl: Some(60),
                label_whitelist: Some(vec!["test".to_owned()]),
            },
            default: true,
            host_pattern: Some(r#"\b(quality|staging)\b.*"#.to_string()),
        };
        let profiles = [
            expected_profile.clone(),
            Profile {
                name: "test2".to_owned(),
                config: Config {
                    username: Some("test2".to_owned()),
                    auth: Some("test2".to_owned()),
                    proxy: Some("test2".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test2".to_owned()]),
                },
                default: false,
                host_pattern: Some(r#"\b(dev|prod)\b.*"#.to_string()),
            },
        ];
        assert_eq!(
            expected_profile,
            Profiles::get_matching(hostname, profiles.to_vec())
                .unwrap()
                .unwrap()
        );
    }
}
