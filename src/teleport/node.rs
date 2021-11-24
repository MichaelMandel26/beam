use std::collections::HashMap;

use pad::PadStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    metadata: Metadata,
    spec: Spec,
}

impl Node {
    pub fn into_skim_string(self) -> String {
        let mut label_string = String::new();
        for (key, value) in &self.metadata.labels {
            label_string.push_str(key);
            label_string.push(':');
            label_string.push_str(value);
            label_string.push(' ');
        }

        let string = format!(
            "{} {}",
            self.spec
                .hostname
                .pad_to_width_with_alignment(30, pad::Alignment::Left),
            label_string
        );
        string
    }
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
    hostname: String,
}
