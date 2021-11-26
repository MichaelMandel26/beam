use crate::ssh;
use crate::teleport::node;
use crate::utils::config::CONFIG;
use crate::ConnectOpts;
use anyhow::{ensure, Result};

pub fn connect(cfg: ConnectOpts, user: Option<String>, clear_cache: bool) -> Result<()> {
    let nodes = node::get(!clear_cache)?;
    ensure!(
        nodes.iter().any(|node| node.spec.hostname == cfg.host),
        "Host not found in teleport"
    );

    let username = match user {
        Some(user) => user,
        None => CONFIG.username.clone().unwrap_or_else(whoami::username),
    };

    clearscreen::clear()?;
    ssh::connect::connect(cfg.host, username)?;

    Ok(())
}
