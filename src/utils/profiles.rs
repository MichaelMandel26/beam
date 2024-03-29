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

impl From<Profiles> for Vec<Profile> {
    fn from(profiles: Profiles) -> Self {
        profiles
            .profiles
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<Profile>>()
    }
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
                let profiles: Vec<Profile> = profiles_map.into();
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
        let mut matched_profiles = vec![];

        for profile in profiles {
            if profile.host_pattern.is_some() {
                let regex = Regex::new(profile.host_pattern.as_ref().unwrap())?;
                if regex.is_match(hostname) {
                    matched_profiles.push(profile);
                }
            }
        }

        match matched_profiles.len() {
            0 => Ok(None),
            1 => Ok(Some(matched_profiles[0].clone())),
            _ => {
                matched_profiles.retain(|profile| profile.priority.is_some());
                matched_profiles.sort_by(|a, b| a.priority.cmp(&b.priority));

                Ok(if matched_profiles.is_empty() {
                    None
                } else {
                    Some(matched_profiles[0].clone())
                })
            }
        }
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
                priority: Some(0),
                config: Config {
                    username: Some("test".to_owned()),
                    auth: Some("test".to_owned()),
                    proxy: Some("test".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test".to_owned()]),
                    enable_port_forwarding: Some(false),
                    listen_port: None,
                    remote_port: None,
                    remote_host: None,
                },
                default: true,
                host_pattern: None,
            },
            Profile {
                name: "test2".to_owned(),
                priority: Some(1),
                config: Config {
                    username: Some("test2".to_owned()),
                    auth: Some("test2".to_owned()),
                    proxy: Some("test2".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test2".to_owned()]),
                    enable_port_forwarding: Some(true),
                    listen_port: Some(3306),
                    remote_port: Some(3306),
                    remote_host: Some("127.0.0.1".to_string()),
                },
                default: false,
                host_pattern: None,
            },
        ];
        let invalid_profiles = [
            Profile {
                name: "test".to_owned(),
                priority: Some(1),
                config: Config {
                    username: Some("test".to_owned()),
                    auth: Some("test".to_owned()),
                    proxy: Some("test".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test".to_owned()]),
                    enable_port_forwarding: Some(false),
                    listen_port: None,
                    remote_port: None,
                    remote_host: None,
                },
                default: true,
                host_pattern: None,
            },
            Profile {
                name: "test2".to_owned(),
                priority: Some(0),
                config: Config {
                    username: Some("test2".to_owned()),
                    auth: Some("test2".to_owned()),
                    proxy: Some("test2".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test2".to_owned()]),
                    enable_port_forwarding: Some(true),
                    listen_port: Some(3306),
                    remote_port: Some(3306),
                    remote_host: Some("127.0.0.1".to_string()),
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
                priority: Some(1),
                config: Config {
                    username: Some("test".to_owned()),
                    auth: Some("test".to_owned()),
                    proxy: Some("test".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test".to_owned()]),
                    enable_port_forwarding: Some(false),
                    listen_port: None,
                    remote_port: None,
                    remote_host: None,
                },
                default: true,
                host_pattern: None,
            },
            Profile {
                name: "test2".to_owned(),
                priority: Some(0),
                config: Config {
                    username: Some("test2".to_owned()),
                    auth: Some("test2".to_owned()),
                    proxy: Some("test2".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test2".to_owned()]),
                    enable_port_forwarding: Some(false),
                    listen_port: None,
                    remote_port: None,
                    remote_host: None,
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
    fn test_get_matching_with_priority() {
        let hostname = "quality.app.example.com";
        let expected_profile = Profile {
            name: "test".to_owned(),
            priority: Some(0),
            config: Config {
                username: Some("test".to_owned()),
                auth: Some("test".to_owned()),
                proxy: Some("test".to_owned()),
                cache_ttl: Some(60),
                label_whitelist: Some(vec!["test".to_owned()]),
                enable_port_forwarding: Some(false),
                listen_port: None,
                remote_port: None,
                remote_host: None,
            },
            default: true,
            host_pattern: Some(r#"\b(quality|staging)\b.*"#.to_string()),
        };
        let profiles = [
            expected_profile.clone(),
            Profile {
                name: "test2".to_owned(),
                priority: Some(0),
                config: Config {
                    username: Some("test2".to_owned()),
                    auth: Some("test2".to_owned()),
                    proxy: Some("test2".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test2".to_owned()]),
                    enable_port_forwarding: Some(false),
                    listen_port: None,
                    remote_port: None,
                    remote_host: None,
                },
                default: false,
                host_pattern: Some(r#"\b(dev|prod)\b.*"#.to_string()),
            },
            Profile {
                name: "test3".to_owned(),
                priority: Some(1),
                config: Config {
                    username: Some("test3".to_owned()),
                    auth: Some("test3".to_owned()),
                    proxy: Some("test3".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test3".to_owned()]),
                    enable_port_forwarding: Some(false),
                    listen_port: None,
                    remote_port: None,
                    remote_host: None,
                },
                default: false,
                host_pattern: Some(r#"\b(quality|staging)\b.*"#.to_string()),
            },
            Profile {
                name: "test4".to_owned(),
                priority: None,
                config: Config {
                    username: Some("test4".to_owned()),
                    auth: Some("test4".to_owned()),
                    proxy: Some("test4".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test4".to_owned()]),
                    enable_port_forwarding: Some(false),
                    listen_port: None,
                    remote_port: None,
                    remote_host: None,
                },
                default: false,
                host_pattern: Some(r#"\b(quality|staging)\b.*"#.to_string()),
            },
        ];
        assert_eq!(
            expected_profile,
            Profiles::get_matching(hostname, profiles.to_vec())
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn test_get_matching_without_priority() {
        let hostname = "quality.app.example.com";
        let expected_profile = Profile {
            name: "test".to_owned(),
            priority: None,
            config: Config {
                username: Some("test".to_owned()),
                auth: Some("test".to_owned()),
                proxy: Some("test".to_owned()),
                cache_ttl: Some(60),
                label_whitelist: Some(vec!["test".to_owned()]),
                enable_port_forwarding: Some(false),
                listen_port: None,
                remote_port: None,
                remote_host: None,
            },
            default: true,
            host_pattern: Some(r#"\b(quality|staging)\b.*"#.to_string()),
        };
        let profiles = [
            expected_profile.clone(),
            Profile {
                name: "test2".to_owned(),
                priority: None,
                config: Config {
                    username: Some("test2".to_owned()),
                    auth: Some("test2".to_owned()),
                    proxy: Some("test2".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test2".to_owned()]),
                    enable_port_forwarding: Some(false),
                    listen_port: None,
                    remote_port: None,
                    remote_host: None,
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

    #[test]
    fn test_get_matching_without_priority_multiple_matching() {
        let hostname = "quality.app.example.com";
        let expected_result: Option<Profile> = None;
        let profiles = [
            Profile {
                name: "test".to_owned(),
                priority: None,
                config: Config {
                    username: Some("test".to_owned()),
                    auth: Some("test".to_owned()),
                    proxy: Some("test".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test".to_owned()]),
                    enable_port_forwarding: Some(false),
                    listen_port: None,
                    remote_port: None,
                    remote_host: None,
                },
                default: true,
                host_pattern: Some(r#"\b(quality|staging)\b.*"#.to_string()),
            },
            Profile {
                name: "test2".to_owned(),
                priority: None,
                config: Config {
                    username: Some("test2".to_owned()),
                    auth: Some("test2".to_owned()),
                    proxy: Some("test2".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test2".to_owned()]),
                    enable_port_forwarding: Some(false),
                    listen_port: None,
                    remote_port: None,
                    remote_host: None,
                },
                default: false,
                host_pattern: Some(r#"\b(quality|staging)\b.*"#.to_string()),
            },
        ];
        assert_eq!(
            expected_result,
            Profiles::get_matching(hostname, profiles.to_vec()).unwrap()
        );
    }

    #[test]
    fn test_get_matching_no_match() {
        let hostname = "quality.app.example.com";
        let expected_result: Option<Profile> = None;
        let profiles = [Profile {
            name: "test".to_owned(),
            priority: None,
            config: Config {
                username: Some("test".to_owned()),
                auth: Some("test".to_owned()),
                proxy: Some("test".to_owned()),
                cache_ttl: Some(60),
                label_whitelist: Some(vec!["test".to_owned()]),
                enable_port_forwarding: Some(false),
                listen_port: None,
                remote_port: None,
                remote_host: None,
            },
            default: false,
            host_pattern: Some(r#"\b(dev|prod)\b.*"#.to_string()),
        }];
        assert_eq!(
            expected_result,
            Profiles::get_matching(hostname, profiles.to_vec()).unwrap()
        );
    }

    #[test]
    fn test_from_profiles() {
        let profiles = Profiles {
            profiles: Map::from([
                (
                    "test".to_string(),
                    Profile {
                        name: "test".to_owned(),
                        priority: Some(0),
                        config: Config {
                            username: Some("test".to_owned()),
                            auth: Some("test".to_owned()),
                            proxy: Some("test".to_owned()),
                            cache_ttl: Some(60),
                            label_whitelist: Some(vec!["test".to_owned()]),
                            enable_port_forwarding: Some(false),
                            listen_port: None,
                            remote_port: None,
                            remote_host: None,
                        },
                        default: true,
                        host_pattern: Some(r#"\b(quality|staging)\b.*"#.to_string()),
                    },
                ),
                (
                    "test2".to_string(),
                    Profile {
                        name: "test2".to_owned(),
                        priority: Some(0),
                        config: Config {
                            username: Some("test2".to_owned()),
                            auth: Some("test2".to_owned()),
                            proxy: Some("test2".to_owned()),
                            cache_ttl: Some(60),
                            label_whitelist: Some(vec!["test2".to_owned()]),
                            enable_port_forwarding: Some(false),
                            listen_port: None,
                            remote_port: None,
                            remote_host: None,
                        },
                        default: false,
                        host_pattern: Some(r#"\b(quality|staging)\b.*"#.to_string()),
                    },
                ),
            ]),
        };

        let expected_profile_vec = vec![
            Profile {
                name: "test".to_owned(),
                priority: Some(0),
                config: Config {
                    username: Some("test".to_owned()),
                    auth: Some("test".to_owned()),
                    proxy: Some("test".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test".to_owned()]),
                    enable_port_forwarding: Some(false),
                    listen_port: None,
                    remote_port: None,
                    remote_host: None,
                },
                default: true,
                host_pattern: Some(r#"\b(quality|staging)\b.*"#.to_string()),
            },
            Profile {
                name: "test2".to_owned(),
                priority: Some(0),
                config: Config {
                    username: Some("test2".to_owned()),
                    auth: Some("test2".to_owned()),
                    proxy: Some("test2".to_owned()),
                    cache_ttl: Some(60),
                    label_whitelist: Some(vec!["test2".to_owned()]),
                    enable_port_forwarding: Some(false),
                    listen_port: None,
                    remote_port: None,
                    remote_host: None,
                },
                default: false,
                host_pattern: Some(r#"\b(quality|staging)\b.*"#.to_string()),
            },
        ];

        let profile_vec: Vec<Profile> = profiles.into();

        assert_eq!(expected_profile_vec, profile_vec);
    }
}
