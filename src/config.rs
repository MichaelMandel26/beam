use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct Config {
    pub username: String,
    pub proxy: String,
    pub auth: String,
    pub cache_ttl: u64,
    // This value is only used for runtime conditions. Should not be persisted in any config file
    #[serde(skip_serializing)]
    pub clear_cache: bool,
    pub label_whitelist: Option<Vec<String>>,
    pub port_forwarding_config: PortForwardingConfig,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
struct PortForwardingConfig {
    pub enabled: bool,
    pub listen_port: Option<u16>,
    pub remote_port: Option<u16>,
    pub remote_host: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct ConfigBuilder {
    pub username: Option<String>,
    pub proxy: String,
    pub auth: String,
    pub cache_ttl: Option<u64>,
    pub clear_cache: bool,
    pub label_whitelist: Option<Vec<String>>,
    pub enable_port_forwarding: Option<bool>,
    pub listen_port: Option<u16>,
    pub remote_port: Option<u16>,
    pub remote_host: Option<String>,
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    pub fn username(mut self, username: impl ToString) -> ConfigBuilder {
        self.username = Some(username.to_string());
        self
    }

    pub fn proxy(mut self, proxy: impl ToString) -> ConfigBuilder {
        self.proxy = proxy.to_string();
        self
    }

    pub fn auth(mut self, auth: impl ToString) -> ConfigBuilder {
        self.auth = auth.to_string();
        self
    }

    pub fn cache_ttl(mut self, cache_ttl: u64) -> ConfigBuilder {
        self.cache_ttl = Some(cache_ttl);
        self
    }

    pub fn label_whitelist(mut self, label_whitelist: Vec<String>) -> ConfigBuilder {
        self.label_whitelist = Some(label_whitelist);
        self
    }

    pub fn clear_cache(mut self, clear_cache: bool) -> ConfigBuilder {
        self.clear_cache = clear_cache;
        self
    }

    pub fn build(&self) -> Config {
        Config {
            username: self.username.unwrap_or(whoami::username()),
            proxy: self.proxy,
            auth: self.auth,
            cache_ttl: self.cache_ttl.unwrap_or(60 * 60 * 24),
            clear_cache: self.clear_cache,
            label_whitelist: self.label_whitelist,
            port_forwarding_config: PortForwardingConfig::default(),
        }
    }
}

impl Config {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}
