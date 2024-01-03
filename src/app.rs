use crate::{
    config::{Config, ConfigVersion},
    context::RuntimeContext,
};
use anyhow::Result;
use clap::Parser;

use crate::command;

#[derive(Parser, Debug)]
#[clap(name = "beam", about = "Easier connection to teleport hosts", version)]
pub struct App {
    #[clap(short, long, help = "The profile to use")]
    pub profile: Option<String>,

    #[clap(
        short,
        long,
        help = "The user which will be used to connect to the host. (default is the current system user)"
    )]
    pub user: Option<String>,

    #[clap(long, help = "The proxy to use")]
    pub proxy: Option<String>,

    #[clap(long, help = "The auth to use")]
    pub auth: Option<String>,

    #[clap(short, long = "clear-cache", help = "Whether to clear the cache")]
    pub clear_cache: bool,

    #[clap(subcommand)]
    pub cmd: Option<Command>,

    #[clap(
        global = true,
        long = "tsh",
        help = "output the tsh command, that would be used to connect to the node"
    )]
    pub tsh: bool,
}

#[derive(Parser, Debug)]
pub enum Command {
    Connect(command::connect::Connect),
    #[structopt(alias = "ls")]
    List(command::list::List),
    Completions(command::completions::Completions),
    Login(command::login::Login),
    Logout(command::logout::Logout),
}

impl App {
    pub async fn run(self) -> Result<()> {
        App::check_for_beam_config_dir()?;
        let context = self.context()?;
        self.execute_command(context)?;
        Ok(())
    }

    pub fn execute_command(&self, context: RuntimeContext) -> Result<()> {
        match &self.cmd {
            Some(Command::Connect(command)) => command.run(context),
            Some(Command::List(command)) => command.run(context),
            Some(Command::Login(command)) => command.run(context),
            Some(Command::Logout(command)) => command.run(),
            Some(Command::Completions(command)) => command.run(),
            None => command::default::Default::run(context),
        }
    }

    pub fn check_for_beam_config_dir() -> Result<()> {
        let home_dir = home::home_dir().expect("Could not find home directory");
        let dot_beam_dir = home_dir.join(".config/beam");

        if !dot_beam_dir.exists() {
            std::fs::create_dir(&dot_beam_dir)?;
        }

        Ok(())
    }

    fn context(&self) -> Result<RuntimeContext> {
        let config_version = Config::find_current_version();

        let config = match config_version {
            ConfigVersion::V1 => {
                Config::migrate(ConfigVersion::V1, ConfigVersion::Default);
                Config::read_default_version()
            }
            ConfigVersion::V2 | ConfigVersion::Default => Config::read_default_version(),
            ConfigVersion::None => {
                // TODO: enhance message here with link to docs
                anyhow::bail!("Could not find any config file")
            }
        };

        let runtime_context = RuntimeContext::builder()
            .with_config(config)
            .with_app(self)
            .build();

        Ok(runtime_context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_for_beam_config_dir() {
        assert!(App::check_for_beam_config_dir().is_ok());
    }
}
