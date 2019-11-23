use std::collections::HashMap;

use serde::Deserialize;

pub type Flora = HashMap<FloraProviderName, FloraProvider>;
pub type FloraProviderName = String;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "provides")]
pub enum FloraProvider {
    #[serde(rename = "app")]
    App {
        setup: Vec<String>,
        teardown: Vec<String>,
    },

    #[serde(rename = "os")]
    Os {
        os: String,
    },
}