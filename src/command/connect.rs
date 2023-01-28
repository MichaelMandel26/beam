use anyhow::{ensure, Result};
use clap::Parser;

use crate::ssh;
use crate::{
    context::RuntimeContext,
    teleport::{cli, node},
};

#[derive(Debug, Parser)]
pub struct Connect {
    #[clap(help = "The host to connect to")]
    host: String,
}

impl Connect {
    pub fn run(&self, context: RuntimeContext) -> Result<()> {
        if !cli::is_logged_in()? || !cli::cmp_logged_in_proxy_with(&context.config.proxy)? {
            let exit_status = cli::login(
                &context.config.proxy,
                context.config.auth,
                &context.config.username,
            )?;
            if !exit_status.success() {
                return Err(anyhow::anyhow!("Login failed"));
            }
        }

        let nodes = node::get(!&context.flags.clear_cache, &context.config.proxy)?;
        ensure!(
            nodes.iter().any(|node| node.spec.hostname == self.host),
            "Host not found in teleport"
        );

        let tsh_args = ssh::connect::get_tsh_command(
            &self.host,
            &context.config.username,
            &context.meta.profile_name,
            &context.config.port_forwarding_config,
        )?;
        if context.flags.tsh {
            println!("{}", tsh_args.join(" "));
            return Ok(());
        }

        clearscreen::clear()?;
        ssh::connect::connect(tsh_args)?;

        Ok(())
    }
}
