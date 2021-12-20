use std::process;

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select};
use structopt::StructOpt;

use crate::utils::profile::Profiles;

#[derive(StructOpt, Debug)]
pub struct Remove {
    #[structopt(help = "The profile to remove")]
    profile: Option<String>,
}

impl Remove {
    pub fn run(&self) -> Result<()> {
        let mut profiles = match Profiles::get() {
            Ok(profiles) => profiles,
            Err(err) => {
                println!("Error: {}", err);
                process::exit(1);
            }
        };

        if profiles.is_empty() {
            println!("No profiles found");
            process::exit(1);
        }

        let profile_name = match &self.profile {
            Some(profile) => {
                if !profiles.iter().any(|p| &p.name == profile) {
                    println!("Profile with name {} does not exist", profile);
                    process::exit(1);
                }
                profile.to_owned()
            }
            None => {
                // Get profile names
                let profile_names = Profiles::get_names(&profiles)?;

                // Select profile
                let profile_name = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select a profile to remove")
                    .default(0)
                    .items(&profile_names)
                    .interact()?;
                profile_names[profile_name]
                    .clone()
                    .split(' ')
                    .next()
                    .unwrap()
                    .to_owned()
            }
        };

        let is_default_profile = profiles.iter().any(|p| p.name == profile_name && p.default);
        if is_default_profile {
            if profiles.len() == 1 {
                println!("Cannot remove default profile, there must be at least one profile");
                process::exit(1);
            }

            println!("You are trying to remove the default profile. Please select a new default profile first.");
            // Get profile names
            let mut profile_names = Profiles::get_names(&profiles)?;
            profile_names.retain(|p| p != &format!("{} (default)", profile_name));

            let new_default_name_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select new default profile")
                .default(0)
                .items(&profile_names)
                .interact()?;

            let new_default_name = profile_names[new_default_name_selection].clone();

            // Set new default profile
            profiles.iter_mut().for_each(|p| {
                if p.name == new_default_name {
                    p.default = true;
                }
            });
        }

        // Remove profile by name from profiles
        profiles.retain(|p| p.name != profile_name);
        let profiles: Profiles = profiles.into();
        match profiles.save() {
            Ok(_) => {
                println!("Profile {} removed", profile_name);
                Ok(())
            }
            Err(err) => {
                println!("Error: {}", err);
                process::exit(1);
            }
        }
    }
}
