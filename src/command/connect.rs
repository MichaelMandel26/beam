use anyhow::{ensure, Result};
use clap::Parser;

use crate::{ssh, config::{Config, RuntimeConfig}};
use crate::teleport::{cli, node};

#[derive(Debug, Parser)]
pub struct Connect {
    #[clap(help = "The host to connect to")]
    host: String,
}

impl Connect {
    pub fn run(&self, config: &RuntimeConfig) -> Result<()> {
        if !cli::is_logged_in()? || !cli::cmp_logged_in_proxy_with(&config.config.proxy)? {
            let exit_status = cli::login(&config.config.proxy, &config.config.auth, &config.config.username)?;
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

        let tsh_args = ssh::connect::get_tsh_command(&self.host, username, &profile)?;
        if beam.tsh {
            println!("{}", tsh_args.join(" "));
            return Ok(());
        }

        clearscreen::clear()?;
        ssh::connect::connect(tsh_args)?;

        Ok(())
    }
}
