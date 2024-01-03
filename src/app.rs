use crate::{
    config::Config,
    context::RuntimeContext,
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
    #[structopt(alias = "ls")]
    List(command::list::List),
    Completions(command::completions::Completions),
    Login(command::login::Login),
    Logout(command::logout::Logout),
}

impl App {
    pub async fn run(self) -> Result<()> {
        App::check_for_beam_config_dir()?;
        // Asynchronously getting the latest version from GitHub
        let latest_version =
            tokio::spawn(async move { version::get_latest_release(LATEST_RELEASE_URL).await });

        let context = self.context()?;

        self.execute_command(context)?;

        // Printing notification if the latest version is newer than the current version
        App::check_for_update(latest_version.await?)?;
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

    pub fn check_for_beam_config_dir() -> Result<()> {
        let home_dir = home::home_dir().expect("Could not find home directory");
        let dot_beam_dir = home_dir.join(".config/beam");

        if !dot_beam_dir.exists() {
            std::fs::create_dir(&dot_beam_dir)?;
        }

        Ok(())
    }

    fn context(&self) -> Result<RuntimeContext> {
        let profile = match &self.profile.is_some() {
            true => Profile::get(self.profile.as_ref().unwrap().as_str())?,
            false => DEFAULT_PROFILE.clone(),
        };

        let username = self.user.as_ref().unwrap_or(&profile.config.username);
        let proxy = self.proxy.as_ref().unwrap_or(&profile.config.proxy);
        let cache_ttl = profile.config.cache_ttl;
        let port_forwarding_config = profile.config.port_forwarding_config;

        let mut config_builder = Config::builder()
            .username(username)
            .proxy(proxy)
            .cache_ttl(cache_ttl)
            .port_forwarding_config(port_forwarding_config.unwrap_or_default());

        let auth = match self.auth {
            Some(_) => self.auth.clone(),
            None => profile.config.auth,
        };

        if let Some(auth) = auth {
            config_builder = config_builder.auth(auth);
        }

        let config = config_builder.build();

        let runtime_context = RuntimeContext::builder()
            .config(config)
            .with_app(self)
            .profile_name(profile.name)
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
