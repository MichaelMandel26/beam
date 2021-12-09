use crate::utils::version;
use anyhow::Result;
use semver::Version;
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

impl Beam {
    pub async fn run(&self) -> Result<()> {
        Beam::check_for_dot_beam_dir()?;
        // Asynchronously getting the latest version from GitHub
        let latest_version = tokio::spawn(async move { version::get_latest_release().await });

        self.execute_command()?;

        // Printing notification if the latest version is newer than the current version
        Beam::check_for_update(latest_version.await??)?;
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

    pub fn check_for_update(latest_version: Version) -> Result<()> {
        let current_version = version::get_current_version();
        if latest_version > current_version {
            println!("A new version of beam is available: {}", latest_version);
        }
        Ok(())
    }

    pub fn check_for_dot_beam_dir() -> Result<()> {
        let home_dir = home::home_dir().expect("Could not find home directory");
        let dot_beam_dir = home_dir.join(".beam");

        if !dot_beam_dir.exists() {
            std::fs::create_dir(&dot_beam_dir)?;
        }

        Ok(())
    }
}
