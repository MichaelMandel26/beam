use anyhow::Result;
use structopt::StructOpt;

pub mod default;
pub mod profile;

#[derive(Debug, StructOpt)]
pub struct Configure {
    #[structopt(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, StructOpt)]
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
