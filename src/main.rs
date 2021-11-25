use anyhow::Result;
use structopt::StructOpt;

mod commands;
mod ssh;
mod teleport;
mod utils;

#[derive(StructOpt, Debug)]
#[structopt(name = "beam", about = "Easier connection to teleport hosts")]
struct Beam {
    #[structopt(
        short,
        long,
        help = "The user which will be used to connect to the host. (default is the current system user)"
    )]
    user: Option<String>,

    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    Connect,
    Config(ConfigOpts),
}

#[derive(StructOpt, Debug, PartialEq, Default)]
pub struct ConfigOpts {
    #[structopt(short, long, help = "The default username to use")]
    username: Option<String>,
    #[structopt(short, long, help = "The TTL for the nodes cache file in seconds")]
    cache_ttl: Option<u64>,
}

fn main() -> Result<()> {
    let beam = Beam::from_args();
    let user = beam.user;

    check_for_dot_beam_dir()?;

    match beam.cmd {
        Some(Command::Connect) => commands::connect::connect(),
        Some(Command::Config(cfg)) => commands::config::config(cfg)?,
        None => commands::default::default(user)?,
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
