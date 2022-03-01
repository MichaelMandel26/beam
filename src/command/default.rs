use anyhow::{Context, Result};
use structopt::StructOpt;

use crate::ssh;
use crate::teleport::node::SkimString;
use crate::teleport::{cli, node};
use crate::utils::profile::Profile;
use crate::utils::profiles::{Profiles, DEFAULT_PROFILE};
use crate::utils::skim;

#[derive(Debug, StructOpt)]
pub struct Default {}

impl Default {
    pub fn run(beam: &crate::cli::Beam) -> Result<()> {
        let profile = match &beam.profile.is_some() {
            true => Profile::get(beam.profile.as_ref().unwrap().as_str())?,
            false => DEFAULT_PROFILE.clone(),
        };

        let proxy = match &beam.proxy {
            Some(proxy) => proxy,
            None => profile.config.proxy.as_ref().context("No proxy configured to login with. Please use --proxy or configure it using beam configure")?
        };

        let fallback = whoami::username();
        let user = match &beam.user {
            Some(user) => user,
            None => profile.config.username.as_ref().context("No username configured to login with. Please use --username or configure it using beam configure").unwrap_or(&fallback)
        };

        let auth = match &beam.auth {
            Some(auth) => Some(auth),
            None => profile.config.auth.as_ref(),
        };

        if !cli::is_logged_in()? || !cli::cmp_logged_in_proxy_with(proxy)? {
            let exit_status = cli::login(proxy, auth, user)?;
            if !exit_status.success() {
                return Err(anyhow::anyhow!("Login failed"));
            }
        }

        let nodes = node::get(!beam.clear_cache, proxy)?;

        let label_whitelist = profile.config.label_whitelist.clone();

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

        clearscreen::clear()?;
        match matched_profile {
            Some(matched_profile) => ssh::connect::connect(
                host,
                matched_profile.config.username.as_ref().unwrap(),
                &matched_profile,
            )?,
            None => ssh::connect::connect(host, user, &profile)?,
        };

        Ok(())
    }
}
