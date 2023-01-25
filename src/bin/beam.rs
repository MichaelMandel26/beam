use anyhow::Result;
use beamcli::cli::App;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    match App::parse().run().await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
