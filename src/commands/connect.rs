use crate::ssh;
use crate::teleport::node;
use crate::utils::config::CONFIG;
use crate::ConnectOpts;
use anyhow::{Result, ensure};

pub fn connect(cfg: ConnectOpts, user: Option<String>) -> Result<()> {
    let nodes = node::get()?;
    ensure!(nodes.iter().any(|node| node.spec.hostname == cfg.host), "Host not found in teleport");

    let username = match user {
        Some(user) => user,
        None => CONFIG.username.clone().unwrap_or_else(whoami::username),
    };

    clearscreen::clear()?;
    ssh::connect::connect(cfg.host, username)?;

    Ok(())
}
