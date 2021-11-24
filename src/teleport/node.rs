use std::collections::HashMap;

use pad::PadStr;
use serde::{Deserialize, Serialize};

/*
{
    "kind": "node",
    "version": "v2",
    "metadata": {
      "name": "161bee39-cf07-4e31-90ba-6593c9f505cb",
      "labels": {
        "application": "upload-api"
      },
      "expires": "2021-11-21T21:13:39.138323586Z",
      "id": 1637528619138847700
    },
    "spec": {
      "addr": "127.0.0.1:3022",
      "public_addr": "teleport.dzefo.de:443",
      "hostname": "h2878479.stratoserver.net",
      "cmd_labels": {
        "hostname": {
          "period": "1m0s",
          "command": [
            "hostname"
          ],
          "result": "h2878479.stratoserver.net"
        }
      },
      "rotation": {
        "current_id": "",
        "started": "0001-01-01T00:00:00Z",
        "last_rotated": "0001-01-01T00:00:00Z",
        "schedule": {
          "update_clients": "0001-01-01T00:00:00Z",
          "update_servers": "0001-01-01T00:00:00Z",
          "standby": "0001-01-01T00:00:00Z"
        }
      },
      "version": "8.0.0"
    }
  },
  {
    "kind": "node",
    "version": "v2",
    "metadata": {
      "name": "c1b3ee09-8e4a-49d4-93b8-95cbcb676f20",
      "labels": {
        "application": "creative-api"
      },
      "expires": "2021-11-21T21:14:15.237191692Z",
      "id": 1637528655246512600
    },
    "spec": {
      "addr": "",
      "hostname": "vmd72245.contaboserver.net",
      "rotation": {
        "current_id": "",
        "started": "0001-01-01T00:00:00Z",
        "last_rotated": "0001-01-01T00:00:00Z",
        "schedule": {
          "update_clients": "0001-01-01T00:00:00Z",
          "update_servers": "0001-01-01T00:00:00Z",
          "standby": "0001-01-01T00:00:00Z"
        }
      },
      "use_tunnel": true,
      "version": "8.0.0"
    }
  }
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    metadata: Metadata,
    spec: Spec,
}

impl Node {
  pub fn into_skim_string(&self) -> String {
    let mut label_string = String::new();
    for (key, value) in &self.metadata.labels {
        label_string.push_str(&key);
        label_string.push_str(":");
        label_string.push_str(&value);
        label_string.push_str(" ");
    }

    let string = format!("{} {}", self.spec.hostname.pad_to_width_with_alignment(30, pad::Alignment::Left), label_string);
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
