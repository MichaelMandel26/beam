use anyhow::Result;
use structopt::StructOpt;

use crate::utils::profile::DEFAULT_PROFILE;

#[derive(StructOpt, Debug)]
pub struct Get {}

impl Get {
    pub fn run(&self) -> Result<()> {
        let config = DEFAULT_PROFILE.config.clone();
        if config != Default::default() {
            println!("{}", toml::to_string_pretty(&config)?);
        } else {
            println!("No config found, or no default profile set.");
        }
        Ok(())
    }
}
