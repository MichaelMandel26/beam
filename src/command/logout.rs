use anyhow::Result;
use clap::Parser;

use crate::teleport::cli;

#[derive(Debug, Parser)]
pub struct Logout {}

impl Logout {
    pub fn run(&self) -> Result<()> {
        cli::logout()?;

        Ok(())
    }
}
