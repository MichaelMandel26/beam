use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
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
                let profile_names = profiles.iter().map(|p| p.name.clone()).collect::<Vec<_>>();
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
        let default_proxy = match &config.proxy {
            Some(proxy) => proxy.to_owned(),
            None => "".to_string(),
        };

        // Proxy
        let proxy: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Proxy")
            .show_default(!&default_proxy.is_empty())
            .default(default_proxy)
            .interact_text()?;

        let default_username = match &config.username {
            Some(username) => username.to_owned(),
            None => whoami::username(),
        };

        // Username
        let username = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Username")
            .default(default_username)
            .interact_text()?;

        let default_auth = match &config.auth {
            Some(auth) => auth.to_owned(),
            None => "default".to_string(),
        };

        // Auth
        let auth = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Authentication Method")
            .default(default_auth)
            .interact_text()?;

        let default_ttl = match &config.cache_ttl {
            Some(cache_ttl) => cache_ttl.to_string(),
            None => ((60 * 60 * 24) as u64).to_string(),
        };

        // Auth
        let cache_ttl = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Cache TTL")
            .default(default_ttl)
            .interact_text()?;

        config.proxy = Some(proxy);
        config.username = Some(username);
        config.auth = if auth != "default" { Some(auth) } else { None };
        config.cache_ttl = Some(cache_ttl.parse::<u64>()?);

        Ok(())
    }
}
