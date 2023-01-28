use crate::{
    config::{Config, RuntimeConfig},
    utils::{profile::Profile, profiles::DEFAULT_PROFILE, version},
};
use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use semver::Version;

use crate::command;

const LATEST_RELEASE_URL: &str = "https://github.com/MichaelMandel26/beam/releases/latest";

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
    Profile(command::profile::Profile),
    #[structopt(alias = "ls")]
    List(command::list::List),
    Completions(command::completions::Completions),
    Configure(command::configure::Configure),
    Login(command::login::Login),
    Logout(command::logout::Logout),
}

impl App {
    pub async fn run(&self) -> Result<()> {
        App::check_for_dot_beam_dir()?;
        // Asynchronously getting the latest version from GitHub
        let latest_version =
            tokio::spawn(async move { version::get_latest_release(LATEST_RELEASE_URL).await });

        let config = self.config()?;

        self.execute_command(config)?;

        // Printing notification if the latest version is newer than the current version
        App::check_for_update(latest_version.await?)?;
        Ok(())
    }

    pub fn execute_command(&self, config: RuntimeConfig) -> Result<()> {
        match &self.cmd {
            Some(Command::Connect(command)) => command.run(&config),
            Some(Command::Profile(command)) => command.run(),
            Some(Command::List(command)) => command.run(config),
            Some(Command::Login(command)) => command.run(config),
            Some(Command::Logout(command)) => command.run(),
            Some(Command::Completions(command)) => command.run(),
            Some(Command::Configure(command)) => command.run(),
            None => command::default::Default::run(config),
        }
    }

    pub fn check_for_update(latest_version: Result<Version>) -> Result<()> {
        let current_version = version::get_current_version();
        if let Ok(latest_version) = latest_version {
            if latest_version > current_version {
                println!(
                    "A new version of beam is available {} -> {}\nTo update run {}",
                    current_version.to_string().red(),
                    latest_version.to_string().green(),
                    "cargo install beamcli".green()
                );
            }
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

    fn config(&self) -> Result<RuntimeConfig> {
        let profile = match &self.profile.is_some() {
            true => Profile::get(self.profile.as_ref().unwrap().as_str())?,
            false => DEFAULT_PROFILE.clone(),
        };

        let username = &self.user.unwrap_or(profile.config.username);
        let proxy = &self.proxy.unwrap_or(profile.config.proxy);
        let auth = &self.auth.unwrap_or(profile.config.auth);
        let cache_ttl = profile.config.cache_ttl;

        let config = Config::new()
            .username(username)
            .proxy(proxy)
            .auth(auth)
            .cache_ttl(cache_ttl)
            .build();

        let runtime_config = RuntimeConfig::new()
            .config(config)
            .tsh(self.tsh)
            .clear_cache(self.clear_cache)
            .build();

        Ok(runtime_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_for_dot_beam_dir() {
        assert!(App::check_for_dot_beam_dir().is_ok());
    }
}
