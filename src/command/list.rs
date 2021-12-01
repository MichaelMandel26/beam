use crate::teleport::cli;
use crate::utils::config::CONFIG;
use anyhow::{Context, Result};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct List {
    #[structopt(short, long, help = "The format to use for the output")]
    format: Option<String>,
}

impl List {
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
        let ls_output = cli::ls(self.format.as_ref())?;

        println!("{}", ls_output);
        Ok(())
    }
}
