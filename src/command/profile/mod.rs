mod add;
mod remove;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Profile {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Getting a configuration value
    Add(add::Add),
    /// Setting a configuration value
    #[clap(alias = "rm")]
    Remove(remove::Remove),
}

impl Profile {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Command::Add(cmd) => cmd.run(),
            Command::Remove(cmd) => cmd.run(),
        }
    }
}
