use anyhow::{Context, Result};
use colored::Colorize;
use std::process::{Command, ExitStatus};

use crate::config::PortForwardingConfig;

pub fn connect(mut tsh_args: Vec<String>) -> Result<ExitStatus> {
    tsh_args.remove(0);
    let mut process = Command::new("tsh").args(tsh_args).spawn()?;

    process.wait().map_err(|e| anyhow::anyhow!(e))
}

pub fn get_tsh_command(
    host: &str,
    username: &str,
    profile_name: &str,
    port_forwarding_config: &PortForwardingConfig,
) -> Result<Vec<String>> {
    let host_string = format!("{username}@{host}");

    let mut args: Vec<String> = vec!["tsh".into(), "ssh".into()];
    let port_forwarding_string;

    if port_forwarding_config.enabled {
        let listen_port = port_forwarding_config.listen_port.context(
            format!(
            "port forwarding was activated for profile {}, but listen_port property was not set",
profile_name.cyan()
        )
            .red(),
        )?;
        let remote_host = port_forwarding_config.remote_host.as_ref().context(
            format!(
            "port forwarding was activated for profile {}, but remote_host property was not set",
profile_name.cyan()
        )
            .red(),
        )?;
        let remote_port = port_forwarding_config.remote_port.context(
            format!(
            "port forwarding was activated for profile {}, but remote_port property was not set",
profile_name.cyan()
        )
            .red(),
        )?;
        args.push("-L".into());
        port_forwarding_string = format!("{listen_port}:{remote_host}:{remote_port}");
        args.push(port_forwarding_string);
    }

    args.push(host_string);

    Ok(args)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_tsh_command_port_forwarding() {
        use super::*;
        use crate::config::PortForwardingConfig;

        let username = "testuser";
        let profile_name = "test";

        let port_forwarding_config = PortForwardingConfig {
            enabled: true,
            listen_port: Some(8080),
            remote_host: Some("localhost".into()),
            remote_port: Some(80),
        };

        let args =
            get_tsh_command("t-test", username, profile_name, &port_forwarding_config).unwrap();

        assert_eq!(args[0], "tsh");
        assert_eq!(args[1], "ssh");
        assert_eq!(args[2], "-L");
        assert_eq!(args[3], "8080:localhost:80");
        assert_eq!(args[4], "testuser@t-test");
    }

    #[test]
    fn test_get_tsh_command() {
        use super::*;
        use crate::config::PortForwardingConfig;

        let username = "testuser";
        let profile_name = "test";

        let port_forwarding_config = PortForwardingConfig {
            enabled: false,
            listen_port: None,
            remote_host: None,
            remote_port: None,
        };

        let args =
            get_tsh_command("t-test", username, profile_name, &port_forwarding_config).unwrap();

        assert_eq!(args[0], "tsh");
        assert_eq!(args[1], "ssh");
        assert_eq!(args[2], "testuser@t-test");
    }
}
