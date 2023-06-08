use anyhow::Result;
use clap::Parser;

use crate::{context::RuntimeContext, teleport::cli};

#[derive(Debug, Parser)]
pub struct Login {}

impl Login {
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

        Ok(())
    }
}
