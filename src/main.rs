use anyhow::Result;
use structopt::StructOpt;

mod commands;
mod ssh;
mod teleport;
mod utils;

#[derive(StructOpt, Debug)]
#[structopt(name = "beam", about = "Easier connection to teleport hosts")]
pub struct Beam {
    #[structopt(
        short,
        long,
        help = "The user which will be used to connect to the host. (default is the current system user)"
    )]
    user: Option<String>,
    #[structopt(short, long, help = "The proxy to use")]
    proxy: Option<String>,
    #[structopt(short, long = "clear-cache", help = "Whether to clear the cache")]
    clear_cache: bool,
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    Connect(ConnectOpts),
    Config(ConfigOpts),
}

#[derive(StructOpt, Debug, PartialEq, Default)]
pub struct ConfigOpts {
    #[structopt(short, long, help = "Whether to reset the specified configs", conflicts_with_all(&["username", "cache_ttl", "proxy"]))]
    reset: bool,
    #[structopt(short, long, help = "The username to use as a default")]
    username: Option<String>,
    #[structopt(short, long, help = "The proxy to use as a default")]
    proxy: Option<String>,
    #[structopt(short, long, help = "The auth method to use as a default")]
    auth: Option<String>,
    #[structopt(short, long, help = "The TTL for the nodes cache file in seconds")]
    cache_ttl: Option<u64>,
}

#[derive(StructOpt, Debug, PartialEq, Default)]
pub struct ConnectOpts {
    #[structopt(help = "The host to connect to")]
    host: String,
    #[structopt(short, long, help = "The proxy to use")]
    proxy: Option<String>,
}

fn main() -> Result<()> {
    let beam = Beam::from_args();

    check_for_dot_beam_dir()?;

    match beam.cmd {
        Some(Command::Connect(cfg)) => {
            let clear_cache = beam.clear_cache;
            let user = beam.user.clone();
            commands::connect::connect(cfg, user, clear_cache)?
        }
        Some(Command::Config(cfg)) => commands::config::config(cfg)?,
        None => commands::default::default(beam)?,
    }

    Ok(())
}

fn check_for_dot_beam_dir() -> Result<()> {
    let home_dir = home::home_dir().expect("Could not find home directory");
    let dot_beam_dir = home_dir.join(".beam");

    if !dot_beam_dir.exists() {
        std::fs::create_dir(&dot_beam_dir)?;
    }

    Ok(())
}
