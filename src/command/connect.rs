use anyhow::{ensure, Context, Result};
use clap::Parser;

use crate::ssh;
use crate::teleport::{cli, node};
use crate::utils::profile::Profile;
use crate::utils::profiles::{Profiles, DEFAULT_PROFILE};

#[derive(Debug, Parser)]
pub struct Connect {
    #[clap(help = "The host to connect to")]
    host: String,
}

impl Connect {
    pub fn run(&self, beam: &crate::cli::Beam) -> Result<()> {
        let profile = match &beam.profile.is_some() {
            true => Profile::get(beam.profile.as_ref().unwrap().as_str())?,
            false => {
                let profiles = Profiles::get()?;
                match Profiles::get_matching(&self.host, profiles)? {
                    Some(p) => p,
                    None => DEFAULT_PROFILE.clone(),
                }
            }
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
        ensure!(
            nodes.iter().any(|node| node.spec.hostname == self.host),
            "Host not found in teleport"
        );

        let fallback = whoami::username();
        let username = match &beam.user {
            Some(username) => username,
            None => profile.config.username.as_ref().context("No username configured to login with. Please use --username or configure it using beam configure").unwrap_or(&fallback)
        };

        clearscreen::clear()?;
        ssh::connect::connect(&self.host, username, &profile)?;

        Ok(())
    }
}
