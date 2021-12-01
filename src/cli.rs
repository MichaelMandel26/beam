use anyhow::Result;
use structopt::StructOpt;

use crate::command;

#[derive(StructOpt, Debug)]
#[structopt(name = "beam", about = "Easier connection to teleport hosts")]
pub struct Beam {
    #[structopt(
        short,
        long,
        help = "The user which will be used to connect to the host. (default is the current system user)"
    )]
    pub user: Option<String>,

    #[structopt(short, long, help = "The proxy to use")]
    pub proxy: Option<String>,

    #[structopt(short, long, help = "The auth to use")]
    pub auth: Option<String>,

    #[structopt(short, long = "clear-cache", help = "Whether to clear the cache")]
    pub clear_cache: bool,

    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Connect(command::connect::Connect),
    Config(command::config::Config),
    List(command::list::List),
}

#[derive(StructOpt, Debug, PartialEq, Default)]
pub struct LsOpts {
    #[structopt(short, long, help = "The format to use for the output")]
    format: Option<String>,
}

impl Beam {
    pub fn run(&self) -> Result<()> {
        self.execute_command()?;

        Ok(())
    }

    pub fn execute_command(&self) -> Result<()> {
        match &self.cmd {
            Some(Command::Connect(command)) => command.run(self),
            Some(Command::Config(command)) => command.run(),
            Some(Command::List(command)) => command.run(self),
            None => command::default::Default::run(self),
        }
    }
}
