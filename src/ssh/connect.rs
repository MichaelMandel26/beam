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

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_tsh_command_port_forwarding() {
        use super::*;
        use crate::utils::config::Config;
        use crate::utils::profile::Profile;

        let username = "testuser";

        let profile = Profile {
            name: "test".into(),
            config: Config {
                username: Some(username.into()),
                enable_port_forwarding: Some(true),
                listen_port: Some(8080),
                remote_host: Some("localhost".into()),
                remote_port: Some(80),
                proxy: None,
                auth: None,
                cache_ttl: None,
                label_whitelist: None,
            },
            default: true,
            host_pattern: None,
            priority: None,
        };

        let args = get_tsh_command("t-test", username, &profile).unwrap();

        assert_eq!(args[0], "tsh");
        assert_eq!(args[1], "ssh");
        assert_eq!(args[2], "-L");
        assert_eq!(args[3], "8080:localhost:80");
        assert_eq!(args[4], "testuser@t-test");
    }

    #[test]
    fn test_get_tsh_command() {
        use super::*;
        use crate::utils::config::Config;
        use crate::utils::profile::Profile;

        let username = "testuser";

        let profile = Profile {
            name: "test".into(),
            config: Config {
                username: Some(username.into()),
                enable_port_forwarding: Some(false),
                listen_port: None,
                remote_host: None,
                remote_port: None,
                proxy: None,
                auth: None,
                cache_ttl: None,
                label_whitelist: None,
            },
            default: true,
            host_pattern: None,
            priority: None,
        };

        let args = get_tsh_command("t-test", username, &profile).unwrap();

        assert_eq!(args[0], "tsh");
        assert_eq!(args[1], "ssh");
        assert_eq!(args[2], "testuser@t-test");
    }
}
