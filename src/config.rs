use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct Config {
    pub username: String,
    pub proxy: String,
    pub auth: Option<String>,
    pub cache_ttl: u64,
    pub label_whitelist: Option<Vec<String>>,
    pub port_forwarding_config: Option<PortForwardingConfig>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct PortForwardingConfig {
    pub enabled: bool,
    pub listen_port: Option<u16>,
    pub remote_port: Option<u16>,
    pub remote_host: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConfigBuilder {
    pub username: Option<String>,
    pub proxy: String,
    pub auth: Option<String>,
    pub cache_ttl: Option<u64>,
    pub label_whitelist: Option<Vec<String>>,
    pub port_forwarding_config: Option<PortForwardingConfig>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder::default()
    }

    pub fn username(mut self, username: impl ToString) -> Self {
        self.username = Some(username.to_string());
        self
    }

    pub fn proxy(mut self, proxy: impl ToString) -> Self {
        self.proxy = proxy.to_string();
        self
    }

    pub fn auth(mut self, auth: impl ToString) -> Self {
        self.auth = Some(auth.to_string());
        self
    }

    pub fn cache_ttl(mut self, cache_ttl: u64) -> Self {
        self.cache_ttl = Some(cache_ttl);
        self
    }

    pub fn label_whitelist(mut self, label_whitelist: Vec<String>) -> Self {
        self.label_whitelist = Some(label_whitelist);
        self
    }

    pub fn port_forwarding_config(mut self, port_forwarding_config: PortForwardingConfig) -> Self {
        self.port_forwarding_config = Some(port_forwarding_config);
        self
    }

    pub fn build(self) -> Config {
        Config {
            username: self.username.unwrap_or(whoami::username()),
            proxy: self.proxy,
            auth: self.auth,
            cache_ttl: self.cache_ttl.unwrap_or(60 * 60 * 24),
            label_whitelist: self.label_whitelist,
            port_forwarding_config: self.port_forwarding_config,
        }
    }
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}
