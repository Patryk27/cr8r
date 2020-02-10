use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcosystemProviderConfig {
    pub setup: Vec<String>,
}