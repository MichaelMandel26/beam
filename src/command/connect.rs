use anyhow::{ensure, Context, Result};
use structopt::StructOpt;

use crate::ssh;
use crate::teleport::{cli, node};
use crate::utils::config::CONFIG;

#[derive(Debug, StructOpt)]
pub struct Connect {
    #[structopt(help = "The host to connect to")]
    host: String,
}

impl Connect {
    pub fn run(&self, beam: &crate::cli::Beam) -> Result<()> {
        let proxy = match &beam.proxy {
            Some(proxy) => proxy,
            None => {
                let proxy = &CONFIG.proxy;
                proxy.as_ref().context("No proxy configured to login with. Please use --proxy or configure it with beam config --proxy <url>")?
            }
        };

        if !cli::is_logged_in()? {
            cli::login(&proxy, beam.auth.as_ref())?;
        }
        let nodes = node::get(!beam.clear_cache, &proxy)?;
        ensure!(
            nodes.iter().any(|node| node.spec.hostname == self.host),
            "Host not found in teleport"
        );

        let fallback = whoami::username();
        let username = match &beam.user {
            Some(username) => username,
            None => &fallback,
        };

        clearscreen::clear()?;
        ssh::connect::connect(&self.host, &username)?;

        Ok(())
    }
}
