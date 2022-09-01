use anyhow::{Context, Result};
use colored::Colorize;
use std::process::{Command, ExitStatus};

use crate::utils::profile::Profile;

pub fn connect(mut tsh_args: Vec<String>) -> Result<ExitStatus> {
    tsh_args.remove(0);
    let mut process = Command::new("tsh").args(tsh_args).spawn()?;

    process.wait().map_err(|e| anyhow::anyhow!(e))
}

pub fn get_tsh_command(host: &str, username: &str, profile: &Profile) -> Result<Vec<String>> {
    let host_string = format!("{}@{}", username, host);

    let mut args: Vec<String> = vec!["tsh".into(), "ssh".into()];
    let port_forwarding_string;

    if profile.config.enable_port_forwarding.is_some()
        && profile.config.enable_port_forwarding.unwrap()
    {
        let listen_port = profile.config.listen_port.context(
            format!(
            "port forwarding was activated for profile {}, but listen_port property was not set",
            profile.name.cyan()
        )
            .red(),
        )?;
        let remote_host = profile.config.remote_host.as_ref().context(
            format!(
            "port forwarding was activated for profile {}, but remote_host property was not set",
            profile.name.cyan()
        )
            .red(),
        )?;
        let remote_port = profile.config.remote_port.context(
            format!(
            "port forwarding was activated for profile {}, but remote_port property was not set",
            profile.name.cyan()
        )
            .red(),
        )?;
        args.push("-L".into());
        port_forwarding_string = format!("{}:{}:{}", listen_port, remote_host, remote_port);
        args.push(port_forwarding_string);
    }

    args.push(host_string);

    Ok(args)
}
