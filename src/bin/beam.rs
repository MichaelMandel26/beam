use anyhow::Result;
use beamcli::cli::Beam;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    match Beam::parse().run().await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
