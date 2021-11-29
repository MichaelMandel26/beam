use anyhow::Result;
use std::process::Command;

use crate::utils;

pub fn is_logged_in() -> Result<bool> {
    let output = Command::new("tsh").args(["status"]).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let is_logged_in = stdout.contains("valid for");
    Ok(is_logged_in)
}

pub fn login(proxy: &str) -> Result<()> {
    Command::new("tsh")
        .args(["login", "--proxy", proxy])
        .output()?;
    Ok(())
}

pub fn ls(format: Option<String>) -> Result<String> {
    let format = match format {
        Some(format) => format,
        None => "text".to_string(),
    };
    let spinner = utils::spinner::get_spinner();
    spinner.set_message("Getting nodes from teleport...");
    let output = Command::new("tsh")
        .args(["ls", "-f", format.as_str()])
        .output()?;

    spinner.finish_with_message("Done");
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
