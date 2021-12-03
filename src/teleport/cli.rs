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
    let proxy_args = format!("--proxy={}", proxy);
    let mut args = vec!["login", proxy_args.as_str()];

    let auth_args;
    if let Some(auth) = auth {
        auth_args = format!("--auth={}", auth);
        args.push(auth_args.as_str());
    } else if CONFIG.auth.is_some() {
        auth_args = format!("--auth={}", CONFIG.auth.as_ref().unwrap());
        args.push(auth_args.as_str());
    }

    let mut process: Child;
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

    spinner.finish_and_clear();
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
