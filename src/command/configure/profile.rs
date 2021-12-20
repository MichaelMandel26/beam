use anyhow::Result;
use structopt::StructOpt;

use super::Configure;
use crate::utils::profile::{self, Profiles};

#[derive(StructOpt, Debug)]
pub struct Profile {
    #[structopt(help = "The profile to configure")]
    profile: String,
}

impl Profile {
    pub fn run(&self) -> Result<()> {
        let mut profile = match self.profile.as_ref() {
            "default" => profile::DEFAULT_PROFILE.clone(),
            "new" => Configure::new_profile(false)?,
            _ => profile::Profile::get(&self.profile)?,
        };
        Configure::wizard(&mut profile.config)?;
        Profiles::write(profile)?;
        Ok(())
    }
}