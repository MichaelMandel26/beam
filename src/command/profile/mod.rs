mod add;
mod remove;

use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Profile {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Getting a configuration value
    Add(add::Add),
    /// Setting a configuration value
    #[structopt(alias = "rm")]
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
