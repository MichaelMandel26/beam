use anyhow::Result;
use std::process::Command;

use crate::teleport::node::Node;
use crate::utils::skim::skim;

pub fn default() -> Result<()> {
    let nodes: Vec<Node> = get_nodes_from_tsh()?;
    dbg!(&nodes);
    let items = nodes.into_iter().map(|node| node.into_skim_string()).collect::<Vec<String>>().join("\n");
    dbg!(&items);
    skim(items);

    Ok(())
}

fn get_nodes_from_tsh() -> Result<Vec<Node>> {
    let tsh_list = Command::new("tsh")
        .args(["ls", "-f", "json"])
        .output()
        .expect("failed to execute process");
    let tsh_list_str = String::from_utf8_lossy(&tsh_list.stdout);
    let tsh_list_json: Vec<Node> = serde_json::from_str(&tsh_list_str)?;
    Ok(tsh_list_json)
}
