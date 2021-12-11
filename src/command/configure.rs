use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use structopt::StructOpt;

use crate::utils::{config::Config, profile::Profile};

#[derive(Debug, StructOpt)]
pub struct Configure {}

impl Configure {
    pub fn run(&self) -> Result<()> {
        let mut profile = Configure::new_profile()?;

        let mut config = Config::get()?.unwrap_or_default();
        Configure::wizard(&mut config)?;

        profile.config = Some(config);

        Ok(())
    }

    fn new_profile() -> Result<Profile> {
        let name = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Profile name")
            .interact_text()?;

        let default = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want this to be your default profile?")
            .interact()?;

        let profile = Profile::new(name, default, None);

        Ok(profile)
    }

    fn wizard(config: &mut Config) -> Result<()> {
        // Proxy
        let proxy: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Proxy")
            .interact_text()?;

        // Username
        let username_default = "dzefo";
        let username = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Username")
            .default(username_default.to_string())
            .interact_text()?;

        // Auth
        let auth = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Authentication Method")
            .default("sso".to_string())
            .interact_text()?;

        // Auth
        let cache_ttl = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Cache TTL")
            .default((60 * 60 * 24 as u64).to_string())
            .interact_text()?;

        config.proxy = Some(proxy);
        config.username = Some(username);
        config.auth = Some(auth);
        config.cache_ttl = Some(cache_ttl.parse::<u64>()?);

        Ok(())
    }
}
