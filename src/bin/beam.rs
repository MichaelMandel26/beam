use anyhow::Result;
use beamcli::cli::Beam;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    match Beam::from_args().run().await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
