use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcosystemProjectConfig {
    #[serde(default)]
    pub requirements: Vec<String>,

    pub repository: String,
}