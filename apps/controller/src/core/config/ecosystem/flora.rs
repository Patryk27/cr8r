use std::collections::HashMap;

use serde::Deserialize;

pub type Flora = HashMap<String, Provider>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provider {
    pub setup: Vec<String>,
}