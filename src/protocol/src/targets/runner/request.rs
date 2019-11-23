use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Request {
    Authenticate {
        name: String,
        secret: String,
    },

    Hello,
}

impl Request {
    pub fn marshal(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn unmarshal(msg: String) -> Self {
        serde_json::from_str(&msg).unwrap()
    }
}

impl Into<String> for Request {
    fn into(self) -> String {
        self.marshal()
    }
}