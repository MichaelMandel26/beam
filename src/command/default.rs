use crate::ssh;
use crate::teleport::{cli, node};
use crate::utils::config::CONFIG;
use crate::utils::skim;
use anyhow::{Context, Result};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Default {}

impl Default {
    pub fn run(beam: &crate::cli::Beam) -> Result<()> {
        let proxy = match &beam.proxy {
            Some(proxy) => proxy,
            None => {
                let proxy = &CONFIG.proxy;
                proxy.as_ref().context("No proxy configured to login with. Please use --proxy or configure it with beam config --proxy <url>")?
            }
        };
        if !cli::is_logged_in()? {
            let exit_status = cli::login(proxy, beam.auth.as_ref())?;
            if !exit_status.success() {
                return Err(anyhow::anyhow!("Login failed"));
            }
        }
        let nodes = node::get(!beam.clear_cache, proxy)?;

        let items = nodes
            .into_iter()
            .map(|node| node.into_skim_string())
            .collect::<Vec<String>>()
            .join("\n");

        let selected_item = match skim::skim(items)? {
            Some(item) => item,
            None => {
                return Ok(());
            }
        };

        let host = selected_item.split(' ').next().unwrap();

        let fallback = whoami::username();
        let username = match &beam.user {
            Some(username) => username,
            None => CONFIG.username.as_ref().unwrap_or(&fallback),
        };

        clearscreen::clear()?;
        ssh::connect::connect(&host.to_string(), username)?;

        Ok(())
    }
}
