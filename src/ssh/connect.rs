use anyhow::Result;
use std::process::{Command, ExitStatus};

pub fn connect(host: &String, username: &String) -> Result<ExitStatus> {
    let host_string = format!("{}@{}", username, host);

    let mut process = Command::new("tsh")
        .args(["ssh", host_string.as_str()])
        .spawn()?;

    process.wait().map_err(|e| anyhow::anyhow!(e))
}
