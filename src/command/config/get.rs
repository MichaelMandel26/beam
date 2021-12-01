use anyhow::Result;
use structopt::StructOpt;

use crate::utils::config::Config;

#[derive(StructOpt, Debug)]
pub struct Get {}

impl Get {
    pub fn run(&self) -> Result<()> {
        let config = Config::get()?.unwrap_or_default();
        println!("{}", config);
        Ok(())
    }
}
