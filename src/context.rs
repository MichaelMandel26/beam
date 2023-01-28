use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct RuntimeFlags {
    pub tsh: bool,
    pub clear_cache: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct RuntimeMeta {
    pub profile_name: String,
    pub is_logged_in: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct RuntimeContext {
    pub flags: RuntimeFlags,
    pub meta: RuntimeMeta,
    pub config: Config,
}

impl RuntimeContext {
    pub fn builder() -> RuntimeContextBuilder {
        RuntimeContextBuilder::new()
    }
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct RuntimeContextBuilder {
    pub flags: RuntimeFlags,
    pub meta: RuntimeMeta,
    pub config: Config,
}

// TODO: maybe write a flasgs builder here
impl RuntimeContextBuilder {
    pub fn new() -> RuntimeContextBuilder {
        RuntimeContextBuilder::default()
    }

    pub fn clear_cache(mut self, clear_cache: bool) -> Self {
        self.flags.clear_cache = clear_cache;
        self
    }

    pub fn tsh(mut self, tsh: bool) -> Self {
        self.flags.tsh = tsh;
        self
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    pub fn profile_name(mut self, profile_name: impl ToString) -> Self {
        self.meta.profile_name = profile_name.to_string();
        self
    }

    pub fn build(self) -> RuntimeContext {
        RuntimeContext {
            meta: self.meta,
            flags: self.flags,
            config: self.config,
        }
    }
}
