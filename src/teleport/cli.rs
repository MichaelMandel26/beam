use std::{process::Command};
use anyhow::Result;

pub fn is_logged_in() -> Result<bool> {
    let output = Command::new("tsh")
        .args(["status"])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let is_logged_in = stdout.contains("valid for");
    Ok(is_logged_in)
}

pub fn login(proxy: String) -> Result<()> {
    Command::new("tsh")
        .args(["login", "--proxy", proxy.as_str()])
        .output()?;
    Ok(())
}

pub fn ls() -> Result<String> {
    let output = Command::new("tsh")
        .args(["ls", "-f", "json"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
