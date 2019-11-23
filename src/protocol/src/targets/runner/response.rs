use actix_web::web::BytesMut;
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Response {
    Authenticate {
        result: Result<(), AuthenticationError>,
    },

    Hello {
        version: Version,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthenticationError {
    IdTaken,
    InvalidSecret,
    NameTaken,
}

impl Response {
    pub fn marshal(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn unmarshal(msg: BytesMut) -> Self {
        serde_json::from_slice(&msg[..]).unwrap()
    }
}

impl Into<String> for Response {
    fn into(self) -> String {
        self.marshal()
    }
}