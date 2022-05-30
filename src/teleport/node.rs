use anyhow::Result;
use itertools::Itertools;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

use crate::teleport::cli;
use crate::utils::profiles::DEFAULT_PROFILE;

pub trait SkimString {
    fn to_skim_string(self, label_whitelist: Option<Vec<String>>) -> String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    metadata: Metadata,
    pub spec: Spec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    name: String,
    labels: HashMap<String, String>,
    expires: String,
    id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spec {
    pub hostname: String,
}

impl SkimString for Vec<Node> {
    fn to_skim_string(self, label_whitelist: Option<Vec<String>>) -> String {
        let mut skim_string = String::new();

        // Get longest hostname length
        let longest_hostname_length = self
            .par_iter()
            .map(|node| node.spec.hostname.len())
            .max()
            .unwrap_or(0);

        // sort nodes by hostname reverse
        let mut nodes = self;
        nodes.sort_by(|a, b| b.spec.hostname.cmp(&a.spec.hostname));

        let label_whitelist = label_whitelist.unwrap_or_default();
        // Generate skim item string
        for node in nodes {
            let mut label_string = String::new();
            for key in node.metadata.labels.keys().sorted() {
                if label_whitelist.is_empty() || label_whitelist.contains(key) {
                    label_string += format!("{}:{} ", key, node.metadata.labels[key]).as_str();
                }
            }

            skim_string += format!(
                "{:<width$} {}\n",
                node.spec.hostname,
                label_string,
                width = longest_hostname_length + 15
            )
            .as_str();
        }

        skim_string
    }
}

pub fn get(use_cache: bool, proxy: &str) -> Result<Vec<Node>> {
    let cache_file = home::home_dir()
        .unwrap()
        .join(format!(".beam/cache/{}.json", proxy));

    let is_cache_file_old = if cache_file.exists() {
        let metadata = cache_file.metadata()?;
        let ttl = DEFAULT_PROFILE.config.cache_ttl.unwrap_or(60 * 60 * 24);
        metadata.modified()?.elapsed()? > Duration::from_secs(ttl)
    } else {
        true
    };

    let nodes = if !std::path::Path::new(&cache_file).exists() || is_cache_file_old || !use_cache {
        get_from_tsh(proxy)?
    } else {
        get_from_cache(proxy)?
    };

    Ok(nodes)
}

fn get_from_tsh(proxy: &str) -> Result<Vec<Node>> {
    let tsh_json = cli::ls(Some(&"json".to_string()))?;
    if tsh_json == "null\n" {
        return Err(anyhow::anyhow!(
            "This proxy does not seem to have any nodes"
        ));
    }
    let tsh_nodes: Vec<Node> = serde_json::from_str(&tsh_json)?;
    write_to_cache(tsh_json, proxy)?;

    Ok(tsh_nodes)
}

fn get_from_cache(proxy: &str) -> Result<Vec<Node>> {
    let cache_path = home::home_dir()
        .unwrap()
        .join(format!(".beam/cache/{}.json", proxy));
    let cache_json = std::fs::read_to_string(cache_path)?;
    let cached_nodes: Vec<Node> = serde_json::from_str(&cache_json)?;
    Ok(cached_nodes)
}

pub fn write_to_cache(nodes_json: String, proxy: &str) -> Result<()> {
    let cache_dir = home::home_dir().unwrap().join(".beam/cache");
    std::fs::create_dir_all(&cache_dir)?;
    let cache_file = cache_dir.join(format!("{}.json", proxy));
    std::fs::write(cache_file, nodes_json)?;
    Ok(())
}
