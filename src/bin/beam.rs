use anyhow::Result;
use beamcli::cli::Beam;
use structopt::StructOpt;

fn main() -> Result<()> {
    check_for_dot_beam_dir()?;
    
    Beam::from_args().run()?;

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
