use anyhow::Result;
use structopt::StructOpt;

mod commands;
mod ssh;
mod teleport;
mod utils;

#[derive(StructOpt, Debug)]
#[structopt(name = "beam", about = "Easier connection to teleport hosts")]
struct Beam {
    #[structopt(long, help = "The user which will be used to connect to the host. (default is the current user)")]
    user: String,

    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    Connect,
}

fn main() -> Result<()> {
    let beam = Beam::from_args();

    match beam.cmd {
        Some(Command::Connect) => commands::connect::connect(),
        None => commands::default::default()?,
    }
    Ok(())
}
