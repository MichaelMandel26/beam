use anyhow::Result;
use std::process::Command;
use std::time::Duration;
use whoami;

use crate::ssh::connect::connect;
use crate::teleport::node::Node;
use crate::utils;
use crate::utils::config::CONFIG;
use crate::utils::skim::skim;

pub fn default(username: Option<String>) -> Result<()> {
    let cache_file = home::home_dir().unwrap().join(".beam/cache/nodes.json");

    let is_cache_file_old = if cache_file.exists() {
        let metadata = cache_file.metadata()?;
        let ttl = CONFIG.cache_ttl.unwrap_or(60 * 60 * 24);
        metadata.modified()?.elapsed()? > Duration::from_secs(ttl)
    } else {
        true
    };

    let nodes: Vec<Node>;
    if !std::path::Path::new(&cache_file).exists() || is_cache_file_old {
        let spinner = utils::spinner::get_spinner();
        spinner.set_message("Getting nodes from teleport...");
        nodes = get_nodes_from_tsh()?;
        spinner.finish_with_message("Done");
    } else {
        nodes = get_nodes_from_cache()?;
    }

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
    connect(host.to_string(), username)?;

    Ok(())
}

fn get_nodes_from_tsh() -> Result<Vec<Node>> {
    let tsh_list = Command::new("tsh")
        .args(["ls", "-f", "json"])
        .output()
        .expect("failed to execute process");
    let tsh_json = String::from_utf8_lossy(&tsh_list.stdout);
    write_node_json_to_cache(tsh_json.to_string())?;
    let tsh_nodes: Vec<Node> = serde_json::from_str(&tsh_json)?;
    Ok(tsh_nodes)
}

fn write_node_json_to_cache(nodes_json: String) -> Result<()> {
    let cache_dir = home::home_dir().unwrap().join(".beam/cache");
    std::fs::create_dir_all(&cache_dir)?;
    let cache_file = cache_dir.join("nodes.json");
    std::fs::write(cache_file, nodes_json)?;
    Ok(())
}

fn get_nodes_from_cache() -> Result<Vec<Node>> {
    let cache_path = home::home_dir().unwrap().join(".beam/cache/nodes.json");
    let cache_json = std::fs::read_to_string(cache_path)?;
    let cached_nodes: Vec<Node> = serde_json::from_str(&cache_json)?;
    Ok(cached_nodes)
}
