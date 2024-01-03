use anyhow::Result;
use clap::Parser;

use crate::{context::RuntimeContext, teleport::cli};

#[derive(Debug, Parser)]
pub struct List {
    #[clap(short, long, help = "The format to use for the output")]
    format: Option<String>,
}

impl List {
    pub fn run(&self, context: RuntimeContext) -> Result<()> {
        if !cli::is_logged_in(&context.config.proxy)? {
            let exit_status = cli::login(
                &context.config.proxy,
                context.config.auth,
                &context.config.username,
            )?;
            if !exit_status.success() {
                return Err(anyhow::anyhow!("Login failed"));
            }
        }
        let ls_output = cli::ls(self.format.as_ref())?;

        println!("{ls_output}");
        Ok(())
    }
}
