use std::process;

use anyhow::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm};
use structopt::StructOpt;

use crate::utils::{
    config::Config,
    profile::{Profile, Profiles},
};

#[derive(StructOpt, Debug)]
pub struct Add {
    #[structopt(help = "The profile to configure")]
    profile: Option<String>,
}

impl Add {
    pub fn run(&self) -> Result<()> {
        let profiles = match Profiles::get() {
            Ok(profiles) => profiles,
            Err(err) => {
                println!("Error: {}", err);
                process::exit(1);
            }
        };

        if self.profile.is_some()
            && profiles
                .iter()
                .any(|p| &p.name == self.profile.as_ref().unwrap())
        {
            println!(
                "Profile with name {} already exists",
                self.profile.as_ref().unwrap().red()
            );
            process::exit(1);
        }

        let force_default = profiles.is_empty();

        let mut profile = match &self.profile {
            Some(profile) => {
                let default = if force_default {
                    println!("No profiles found, creating default profile");
                    force_default
                } else {
                    Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Do you want this to be your new default profile?")
                        .interact()?
                };
                Profile::new(profile.to_owned(), default, None, Some(Config::default()))
            }
            None => Profile::new_interactive(force_default)?,
        };
        Profile::wizard(&mut profile)?;
        Profiles::write(profile)?;
        Ok(())
    }
}
