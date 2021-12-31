use anyhow::Result;
use colored::Colorize;
use std::process::{Child, Command, ExitStatus};

use crate::utils::spinner;

pub fn is_logged_in() -> Result<bool> {
    let output = match Command::new("tsh").args(["status"]).output() {
        Ok(output) => output,
        Err(_) => {
            return Err(anyhow::anyhow!("Unable to access the teleport cli.\nPlease make sure, that the teleport cli is installed and available in your PATH. For further information see the teleport documentation: https://goteleport.com/docs/installation/".red()));
        }
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    let is_logged_in = stdout.contains("valid for");
    Ok(is_logged_in)
}

pub fn login(proxy: &str, auth: Option<&String>, user: &str) -> Result<ExitStatus> {
    let proxy_args = format!("--proxy={}", proxy);
    let mut args = vec!["login", proxy_args.as_str()];

    let user_args = format!("--user={}", user);
    args.push(user_args.as_str());

    let auth_args;
    if auth.is_some() {
        auth_args = format!("--auth={}", auth.unwrap());
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
    let spinner = spinner::get_spinner();
    spinner.set_message("Getting nodes from teleport...");
    let output = Command::new("tsh").args(["ls", "-f", format]).output()?;

    spinner.finish_and_clear();
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn cmp_logged_in_proxy_with(proxy: &str) -> Result<bool> {
    let output = Command::new("tsh").args(["status"]).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let logged_in_proxy = stdout
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .nth(3)
        .unwrap();
    let proxy = format!("https://{}:443", proxy.split(':').next().unwrap());
    Ok(proxy == logged_in_proxy)
}
