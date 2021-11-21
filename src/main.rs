use anyhow::Result;
use structopt::StructOpt;

mod commands;
mod teleport;
mod utils;

#[derive(StructOpt, Debug)]
#[structopt(name = "beam", about = "Easier connection to teleport hosts")]
struct Beam {
    #[structopt(long, help = "Toogles verbose mode")]
    verbose: bool,
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
