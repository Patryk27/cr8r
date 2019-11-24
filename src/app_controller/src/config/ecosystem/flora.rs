use std::collections::HashMap;

use serde::Deserialize;

pub type Flora = HashMap<ProviderName, Provider>;
pub type ProviderName = String;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "provides")]
pub struct Provider {
    pub setup: Vec<String>,
}