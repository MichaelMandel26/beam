use anyhow::Result;
use clap::Parser;

use crate::utils::profile;
use crate::utils::profiles::{Profiles, DEFAULT_PROFILE};

#[derive(Parser, Debug)]
pub struct Profile {
    #[clap(help = "The profile to configure")]
    profile: String,
}

impl Profile {
    pub fn run(&self) -> Result<()> {
        let mut profile = match self.profile.as_ref() {
            "default" => DEFAULT_PROFILE.clone(),
            "new" => profile::Profile::new_interactive(false)?,
            _ => profile::Profile::get(&self.profile)?,
        };
        profile::Profile::wizard(&mut profile)?;
        Profiles::write(profile)?;
        Ok(())
    }
}
