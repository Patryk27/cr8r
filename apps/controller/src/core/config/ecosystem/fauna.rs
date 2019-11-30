use std::collections::HashMap;

use serde::Deserialize;

pub type Fauna = HashMap<ProjectName, Project>;
pub type ProjectName = String;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub requirements: Vec<String>,
    pub repository: String,
}