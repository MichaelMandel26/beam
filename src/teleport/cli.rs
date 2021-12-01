use crate::utils::config::CONFIG;
use anyhow::Result;
use std::process::Child;
use std::process::Command;
use std::process::ExitStatus;

use crate::utils;

pub fn is_logged_in() -> Result<bool> {
    let output = Command::new("tsh").args(["status"]).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let is_logged_in = stdout.contains("valid for");
    Ok(is_logged_in)
}

pub fn login(proxy: &str, auth: Option<&String>) -> Result<ExitStatus> {
    let mut args = vec!["login", "--proxy", proxy];

    let mut process: Child;
    if let Some(ref auth) = auth {
        args.push("--auth");
        args.push(auth);
    } else if CONFIG.auth.is_some() {
        args.push("--auth");
        args.push(CONFIG.auth.as_ref().unwrap());
    }
    process = Command::new("tsh").args(args).spawn()?;
    process.wait().map_err(|e| anyhow::anyhow!(e))
}

pub fn ls(format: Option<&String>) -> Result<String> {
    let format = match format {
        Some(format) => format,
        None => "text",
    };
    let spinner = utils::spinner::get_spinner();
    spinner.set_message("Getting nodes from teleport...");
    let output = Command::new("tsh").args(["ls", "-f", format]).output()?;

    spinner.finish_with_message("Done");
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
