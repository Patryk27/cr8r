use std::collections::HashMap;

use serde::Deserialize;

pub type Fauna = HashMap<String, Project>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    #[serde(default)]
    pub requirements: Vec<String>,
    pub repository: String,
}