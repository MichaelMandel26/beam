use anyhow::Result;
use beamcli::cli::Beam;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    Beam::from_args().run().await?;

    Ok(())
}
