mod get;
mod set;
mod unset;

use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Getting a configuration value
    Get(get::Get),
    /// Setting a configuration value
    Set(set::Set),
    /// Unsetting a configuration value
    Unset(unset::Unset),
}

impl Config {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Command::Get(cmd) => cmd.run(),
            Command::Set(cmd) => cmd.run(),
            Command::Unset(cmd) => cmd.run(),
        }
    }
}
