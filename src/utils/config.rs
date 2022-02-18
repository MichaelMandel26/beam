use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct Config {
    pub username: Option<String>,
    pub proxy: Option<String>,
    pub auth: Option<String>,
    pub cache_ttl: Option<u64>,
    pub label_whitelist: Option<Vec<String>>,
    pub enable_port_forwarding: Option<bool>,
    pub listen_port: Option<u16>,
    pub remote_port: Option<u16>,
    pub remote_host: Option<String>,
}
