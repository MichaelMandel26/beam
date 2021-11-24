use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;

use crate::teleport::node::Node;
use crate::utils::skim::skim;

pub fn default() -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(80);
    pb.set_style(
        ProgressStyle::default_spinner()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.blue} {msg}"),
    );

    pb.set_message("Getting nodes from teleport...");
    let nodes: Vec<Node> = get_nodes_from_tsh()?;
    pb.finish_with_message("Done");
    let items = nodes
        .into_iter()
        .map(|node| node.into_skim_string())
        .collect::<Vec<String>>()
        .join("\n");
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
