use anyhow::Result;
use structopt::StructOpt;

use crate::utils::config::Config;

#[derive(StructOpt, Debug)]
pub struct Unset {
    #[structopt(short, long, help = "The username to use as a default")]
    username: Option<String>,
    #[structopt(short, long, help = "The proxy to use as a default")]
    proxy: Option<String>,
    #[structopt(short, long, help = "The auth method to use as a default")]
    auth: Option<String>,
    #[structopt(short, long, help = "The TTL for the nodes cache file in seconds")]
    cache_ttl: Option<u64>,
}

impl Unset {
    pub fn run(&self) -> Result<()> {
        let mut config = Config::get()?.unwrap_or_default();

        if self.username.is_some() {
            config.username = None;
        }

        if let Some(_cache_ttl) = &self.cache_ttl {
            config.cache_ttl = None;
        }

        if let Some(_) = &self.proxy {
            config.proxy = None;
        }

        if let Some(_auth) = &self.auth {
            config.auth = None;
        }

        config.write()?;

        Ok(())
    }
}
