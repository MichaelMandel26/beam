use anyhow::Result;
use clap::Parser;

pub mod default;
pub mod profile;

#[derive(Debug, Parser)]
pub struct Configure {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Configure profile
    Profile(profile::Profile),
}

impl Configure {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Some(Command::Profile(cmd)) => cmd.run(),
            None => default::Default::run(),
        }
    }
}
