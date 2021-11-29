use crate::teleport::node;
use crate::utils::config::CONFIG;
use crate::ConnectOpts;
use crate::{ssh, teleport::cli};
use anyhow::{ensure, Context, Result};

pub fn connect(
    cfg: ConnectOpts,
    user: Option<String>,
    clear_cache: bool,
    proxy: Option<String>,
) -> Result<()> {
    let proxy = match proxy {
        Some(proxy) => proxy,
        None => CONFIG.proxy.clone().context("No proxy configured to login with. Please use --proxy or configure it with beam config --proxy <url>")?,
    };
    if !cli::is_logged_in()? {
        cli::login(&proxy)?;
    }
    let nodes = node::get(!clear_cache, proxy)?;
    ensure!(
        nodes.iter().any(|node| node.spec.hostname == cfg.host),
        "Host not found in teleport"
    );

    let username = match user {
        Some(user) => user,
        None => CONFIG.username.clone().unwrap_or_else(whoami::username),
    };

    // clearscreen::clear()?;
    ssh::connect::connect(cfg.host, username)?;

    Ok(())
}
