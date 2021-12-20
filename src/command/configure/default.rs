use std::process;

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select};
use structopt::StructOpt;

use crate::utils::profile::Profiles;

use super::Configure;

#[derive(Debug, StructOpt)]
pub struct Default {}

impl Default {
    pub fn run() -> Result<()> {
        let profiles = match Profiles::get() {
            Ok(profiles) => profiles,
            Err(err) => {
                println!("Error: {}", err);
                process::exit(1);
            }
        };
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
}