use anyhow::Result;
use whoami;

use crate::ssh;
use crate::teleport::node;
use crate::utils::config::CONFIG;
use crate::utils::skim::skim;

pub fn default(username: Option<String>) -> Result<()> {
    let nodes = node::get()?;

    let items = nodes
        .into_iter()
        .map(|node| node.into_skim_string())
        .collect::<Vec<String>>()
        .join("\n");

    let selected_item = match skim(items)? {
        Some(item) => item,
        None => {
            return Ok(());
        }
    };

    let host = selected_item.split(' ').next().unwrap();

    let username = match username {
        Some(username) => username,
        None => CONFIG.username.clone().unwrap_or_else(whoami::username),
    };

    clearscreen::clear()?;
    ssh::connect::connect(host.to_string(), username)?;

    Ok(())
}
