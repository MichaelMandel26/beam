use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap as Map, fs, process};

use crate::{command, utils::config::Config};

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
        let profiles_map = Profiles::get_profiles();
        match profiles_map {
            Ok(profiles_map) => {
                let profiles = profiles_map
                    .profiles
                    .into_iter()
                    .map(|(_, v)| v)
                    .collect::<Vec<Profile>>();
                Profiles::verify_default(&profiles)?;
                Ok(profiles)
            }
            Err(_) => Ok(vec![]),
        }
    }

    pub fn get_default() -> Result<Profile> {
        let profiles = Profiles::get()?;
        Profiles::verify_default(&profiles)
    }

    pub fn verify_default(profiles: &[Profile]) -> Result<Profile> {
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

    pub fn get_matching(hostname: &str) -> Result<Option<Profile>> {
        let profiles = Profiles::get()?;
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

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Profile {
    #[serde(skip)]
    pub name: String,
    pub default: bool,
    pub host_pattern: Option<String>,
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
            config: config.unwrap_or_default(),
        }
    }

    pub fn new_interactive(force_default: bool) -> Result<Profile> {
        let profiles = match Profiles::get() {
            Ok(profiles) => profiles,
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        };

        let name: String = loop {
            let name = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Profile name")
                .interact_text()?;

            if !profiles.iter().any(|p| p.name == name) {
                break name;
            } else {
                println!(
                    "Profile with name {} already exists. Please try a different name",
                    name.red()
                );
            }
        };

        let default = if !force_default {
            Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want this to be your new default profile?")
                .interact()?
        } else {
            true
        };

        let profile = Profile::new(name, default, None, None);

        Ok(profile)
    }

    pub fn wizard(profile: &mut Profile) -> Result<()> {
        profile.host_pattern = Profile::host_pattern_wizard(&profile.host_pattern)?;

        // Proxy
        let default_proxy = match &profile.config.proxy {
            Some(proxy) => proxy.to_owned(),
            None => "".to_string(),
        };

        let proxy: String = if default_proxy.is_empty() {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Proxy")
                .interact_text()?
        } else {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Proxy")
                .default(default_proxy)
                .interact_text()?
        };

        // Username
        let default_username = match &profile.config.username {
            Some(username) => username.to_owned(),
            None => whoami::username(),
        };

        let username = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Username")
            .default(default_username)
            .interact_text()?;

        // Auth
        let default_auth = match &profile.config.auth {
            Some(auth) => auth.to_owned(),
            None => "default".to_string(),
        };

        let auth = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Authentication Method")
            .default(default_auth)
            .interact_text()?;

        // cache_ttl
        let default_ttl = match &profile.config.cache_ttl {
            Some(cache_ttl) => cache_ttl.to_string(),
            None => ((60 * 60 * 24) as u64).to_string(),
        };

        let cache_ttl = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Cache TTL")
            .default(default_ttl)
            .interact_text()?;

        // Label Whitelist
        if let Some(label_whitelist) = &profile.config.label_whitelist {
            let should_edit_label_whitelist = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to edit your label whitelist?")
                .interact()?;

            if should_edit_label_whitelist {
                let action = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("What do you want to do?")
                    .default(0)
                    .items(&["Add label", "Remove label"])
                    .interact()
                    .unwrap();

                let new_label_whitelist: Vec<String> = match action {
                    0 => {
                        let labels_to_add = Profile::label_wizard()?;
                        let mut new_label_whitelist = label_whitelist.clone();
                        new_label_whitelist.extend(labels_to_add);
                        new_label_whitelist
                    }
                    1 => {
                        let defaults = vec![true; label_whitelist.len()];
                        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Choose the labels you want to see. You can toggle a label with the spacebar and submit them with pressing enter.")
                    .items(&label_whitelist[..])
                    .defaults(&defaults[..])
                    .interact()
                    .unwrap();
                        let new_label_whitelist: Vec<String> = label_whitelist
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| selections.contains(i))
                            .map(|(i, _)| label_whitelist[i].clone())
                            .collect();
                        new_label_whitelist
                    }
                    _ => unreachable!(),
                };
                profile.config.label_whitelist = if !new_label_whitelist.is_empty() {
                    Some(new_label_whitelist)
                } else {
                    None
                };
            }
        } else {
            let should_activate_label_whitelist = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to only show specific labels?")
                .interact()?;

            if should_activate_label_whitelist {
                let labels = Profile::label_wizard()?;
                profile.config.label_whitelist = Some(labels);
            }
        }

        profile.config.proxy = Some(proxy);
        profile.config.username = Some(username);
        profile.config.auth = if auth != "default" { Some(auth) } else { None };
        profile.config.cache_ttl = Some(cache_ttl.parse::<u64>()?);

        Ok(())
    }

    pub fn label_wizard() -> Result<Vec<String>> {
        let mut labels = Vec::new();
        loop {
            let label: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Label")
                .interact_text()?;

            if !labels.contains(&label) {
                labels.push(label);
            } else {
                println!(
                    "Label {} is already whitelisted. Please try a different label",
                    label.red()
                );
            }

            let add_another = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Add another label?")
                .interact()?;

            if !add_another {
                break;
            }
        }
        Ok(labels)
    }

    pub fn host_pattern_wizard(default: &Option<String>) -> Result<Option<String>> {
        let confirmation_message = match default {
            Some(default) => format!("Do you want to auto-select this profile, using a regex pattern on the hostname?\n Currently: {}", default),
            None => "Do you want to auto-select this profile, using a regex pattern on the hostname?".to_string(),
        };

        let should_activate_profile_by_pattern = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(confirmation_message)
            .interact()?;

        let host_pattern = if should_activate_profile_by_pattern {
            let host_pattern_input: String = match default {
                Some(default_host_pattern) => Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Regex Pattern for auto-selecting profile")
                    .default(default_host_pattern.to_owned())
                    .interact_text()?,
                None => Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Regex Pattern for auto-selecting profile")
                    .interact_text()?,
            };

            Some(host_pattern_input)
        } else {
            None
        };
        Ok(host_pattern)
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
