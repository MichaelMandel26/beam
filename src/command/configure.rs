use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use structopt::StructOpt;

use crate::utils::{
    config::Config,
    profile::{Profile, Profiles},
};

#[derive(Debug, StructOpt)]
pub struct Configure {}

impl Configure {
    pub fn run(&self) -> Result<()> {
        let profiles = Profiles::get()?;

        let mut profile = match profiles.len().cmp(&1) {
            std::cmp::Ordering::Greater => {
                let profile_names = profiles
                    .iter()
                    .map(|p| {
                        if p.default {
                            format!("{} (default)", p.name)
                        } else {
                            p.name.clone()
                        }
                    })
                    .collect::<Vec<_>>();
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Please select the profile you want to configure")
                    .default(0)
                    .items(&profile_names[..])
                    .interact()
                    .unwrap();
                profiles[selection].clone()
            }
            std::cmp::Ordering::Equal => profiles[0].clone(),
            std::cmp::Ordering::Less => Configure::new_profile(true)?,
        };

        Configure::wizard(&mut profile.config)?;

        Profiles::write(profile)?;

        Ok(())
    }

    fn new_profile(force_default: bool) -> Result<Profile> {
        let name = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Profile name")
            .interact_text()?;

        let default = if !force_default {
            Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want this to be your new default profile?")
                .interact()?
        } else {
            true
        };

        let profile = Profile::new(name, default, None);

        Ok(profile)
    }

    fn wizard(config: &mut Config) -> Result<()> {
        // Proxy
        let default_proxy = match &config.proxy {
            Some(proxy) => proxy.to_owned(),
            None => "".to_string(),
        };

        let proxy: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Proxy")
            .show_default(!&default_proxy.is_empty())
            .default(default_proxy)
            .interact_text()?;

        // Username
        let default_username = match &config.username {
            Some(username) => username.to_owned(),
            None => whoami::username(),
        };

        let username = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Username")
            .default(default_username)
            .interact_text()?;

        // Auth
        let default_auth = match &config.auth {
            Some(auth) => auth.to_owned(),
            None => "default".to_string(),
        };

        let auth = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Authentication Method")
            .default(default_auth)
            .interact_text()?;

        // cache_ttl
        let default_ttl = match &config.cache_ttl {
            Some(cache_ttl) => cache_ttl.to_string(),
            None => ((60 * 60 * 24) as u64).to_string(),
        };

        let cache_ttl = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Cache TTL")
            .default(default_ttl)
            .interact_text()?;

        // Label Whitelist
        if let Some(label_whitelist) = &config.label_whitelist {
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
                        let labels_to_add = Configure::labels()?;
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
                config.label_whitelist = if !new_label_whitelist.is_empty() {
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
                let labels = Configure::labels()?;
                config.label_whitelist = Some(labels);
            }
        }

        config.proxy = Some(proxy);
        config.username = Some(username);
        config.auth = if auth != "default" { Some(auth) } else { None };
        config.cache_ttl = Some(cache_ttl.parse::<u64>()?);

        Ok(())
    }

    pub fn labels() -> Result<Vec<String>> {
        let mut labels = Vec::new();
        loop {
            let label = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Label")
                .interact_text()?;

            labels.push(label);

            let add_another = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Add another label?")
                .interact()?;

            if !add_another {
                break;
            }
        }
        Ok(labels)
    }
}
