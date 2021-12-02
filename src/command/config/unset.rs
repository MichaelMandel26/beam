use anyhow::Result;
use structopt::StructOpt;

use crate::utils::config::Config;

#[derive(StructOpt, Debug)]
pub struct Unset {
    #[structopt(short, long, help = "The username to use as a default")]
    username: bool,
    #[structopt(short, long, help = "The proxy to use as a default")]
    proxy: bool,
    #[structopt(short, long, help = "The auth method to use as a default")]
    auth: bool,
    #[structopt(short, long, help = "The TTL for the nodes cache file in seconds")]
    cache_ttl: bool,
}

impl Unset {
    pub fn run(&self) -> Result<()> {
        let mut config = Config::get()?.unwrap_or_default();

        if self.username {
            config.username = None;
        }

        if self.cache_ttl {
            config.cache_ttl = None;
        }

        if self.proxy {
            config.proxy = None;
        }

        if self.auth {
            config.auth = None;
        }

        config.write()?;

        Ok(())
    }
}