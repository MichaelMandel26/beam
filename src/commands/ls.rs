use crate::teleport;
use crate::teleport::cli;
use crate::utils::config::CONFIG;
use crate::LsOpts;
use anyhow::{Context, Result};

pub fn ls(cfg: LsOpts, proxy: Option<String>, auth: Option<String>) -> Result<()> {
    let proxy = match proxy {
        Some(proxy) => proxy,
        None => CONFIG.proxy.clone().context("No proxy configured to login with. Please use --proxy or configure it with beam config --proxy <url>")?,
    };
    if !cli::is_logged_in()? {
        cli::login(&proxy, auth)?;
    }
    let ls_output = teleport::cli::ls(cfg.format)?;

    println!("{}", ls_output);
    Ok(())
}
