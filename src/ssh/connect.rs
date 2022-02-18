use anyhow::{Context, Result};
use colored::Colorize;
use std::process::{Command, ExitStatus};

use crate::utils::profile::Profile;

pub fn connect(host: &str, username: &str, profile: &Profile) -> Result<ExitStatus> {
    let host_string = format!("{}@{}", username, host);

    let mut args = vec!["ssh"];
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
        args.push("-L");
        port_forwarding_string = format!("{}:{}:{}", listen_port, remote_host, remote_port);
        args.push(port_forwarding_string.as_str());
    }

    args.push(host_string.as_str());

    let mut process = Command::new("tsh").args(args).spawn()?;

    process.wait().map_err(|e| anyhow::anyhow!(e))
}
