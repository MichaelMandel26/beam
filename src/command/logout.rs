use anyhow::Result;
use colored::Colorize;
use structopt::StructOpt;

use crate::teleport::cli;

#[derive(Debug, StructOpt)]
pub struct Logout {}

impl Logout {
    pub fn run(&self) -> Result<()> {
        if cli::is_logged_in()? {
            cli::logout()?;
        } else {
            println!("{}", "You are not logged in with any proxy".red());
        }

        Ok(())
    }
}
