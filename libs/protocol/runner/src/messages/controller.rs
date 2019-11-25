use serde::{Deserialize, Serialize};

use lib_protocol_core::ExperimentReport;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ControllerMessage {
    Authenticate {
        name: String,
        secret: String,
    },

    Hello,

    Report {
        report: ExperimentReport,
    },

    Unpark,
}

impl ControllerMessage {
    pub fn marshal(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn unmarshal(message: String) -> Self {
        serde_json::from_str(&message).unwrap()
    }
}

impl Into<String> for ControllerMessage {
    fn into(self) -> String {
        self.marshal()
    }
}