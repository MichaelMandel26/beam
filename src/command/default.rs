use anyhow::Result;
use clap::Parser;

use crate::ssh;
use crate::teleport::{cli, node};
use crate::utils::profiles::Profiles;
use crate::utils::skim;
use crate::{context::RuntimeContext, teleport::node::SkimString};

#[derive(Debug, Parser)]
pub struct Default {}

impl Default {
    pub fn run(context: RuntimeContext) -> Result<()> {
        if !cli::is_logged_in(&context.config.proxy)? {
            let exit_status = cli::login(
                &context.config.proxy,
                context.config.auth,
                &context.config.username,
            )?;
            if !exit_status.success() {
                return Err(anyhow::anyhow!("Login failed"));
            }
        }

        let nodes = node::get(!context.flags.clear_cache, &context.config.proxy)?;

        let label_whitelist = context.config.label_whitelist.clone();

        let items = nodes.to_skim_string(label_whitelist);

        let selected_item = match skim::skim(items)? {
            Some(item) => item,
            None => {
                return Ok(());
            }
        };

        let host = selected_item.split(' ').next().unwrap();
        let profiles = Profiles::get()?;
        let matched_profile = Profiles::get_matching(host, profiles)?;

        match matched_profile {
            Some(matched_profile) => {
                let tsh_args = ssh::connect::get_tsh_command(
                    host,
                    &matched_profile.config.username,
                    &matched_profile.name,
                    &matched_profile
                        .config
                        .port_forwarding_config
                        .unwrap_or_default(),
                )?;
                if context.flags.tsh {
                    println!("{}", tsh_args.join(" "));
                    return Ok(());
                }
                clearscreen::clear()?;
                ssh::connect::connect(tsh_args)?
            }
            None => {
                let tsh_args = ssh::connect::get_tsh_command(
                    host,
                    &context.config.username,
                    &context.meta.profile_name,
                    &context.config.port_forwarding_config.unwrap_or_default(),
                )?;
                if context.flags.tsh {
                    println!("{}", tsh_args.join(" "));
                    return Ok(());
                }
                clearscreen::clear()?;
                ssh::connect::connect(tsh_args)?
            }
        };

        Ok(())
    }
}
