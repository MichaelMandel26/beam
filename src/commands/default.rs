use anyhow::Result;
use whoami;

use crate::teleport::node;
use crate::utils::config::CONFIG;
use crate::utils::skim::skim;
use crate::{ssh, Beam};

pub fn default(beam: Beam) -> Result<()> {
    let nodes = node::get(!beam.clear_cache)?;

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

    let username = match beam.user {
        Some(username) => username,
        None => CONFIG.username.clone().unwrap_or_else(whoami::username),
    };

    clearscreen::clear()?;
    ssh::connect::connect(host.to_string(), username)?;

    Ok(())
}
