use semver::Version;
use serde::{Deserialize, Serialize};

use lib_protocol_core::{ExperimentId, Scenario};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum RunnerMessage {
    Authenticate {
        result: Result<(), AuthenticationError>,
    },

    Hello {
        version: Version,
    },

    LaunchExperiment {
        id: ExperimentId,
        scenarios: Vec<Scenario>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthenticationError {
    IdTaken,
    InvalidSecret,
    NameTaken,
}

impl RunnerMessage {
    pub fn marshal(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn unmarshal(message: &[u8]) -> Self {
        serde_json::from_slice(message).unwrap()
    }
}

impl Into<String> for RunnerMessage {
    fn into(self) -> String {
        self.marshal()
    }
}