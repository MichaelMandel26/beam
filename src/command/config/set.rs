use anyhow::Result;
use structopt::StructOpt;

use crate::utils::config::Config;

#[derive(StructOpt, Debug)]
pub struct Set {
    #[structopt(short, long, help = "The username to use as a default")]
    username: Option<String>,
    #[structopt(short, long, help = "The proxy to use as a default")]
    proxy: Option<String>,
    #[structopt(short, long, help = "The auth method to use as a default")]
    auth: Option<String>,
    #[structopt(short, long, help = "The TTL for the nodes cache file in seconds")]
    cache_ttl: Option<u64>,
}

impl Set {
    pub fn run(&self) -> Result<()> {
        let mut config = Config::get()?.unwrap_or_default();

        if let Some(username) = &self.username {
            config.username = Some(username.clone());
        }

        if let Some(cache_ttl) = &self.cache_ttl {
            config.cache_ttl = Some(*cache_ttl);
        }

        if let Some(proxy) = &self.proxy {
            config.proxy = Some(proxy.clone());
        }

        if let Some(auth) = &self.auth {
            config.auth = Some(auth.clone());
        }

        config.write()?;

        Ok(())
    }
}
