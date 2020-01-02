use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControllerConfig {
    pub address: String,

    #[serde(default)]
    pub secret: Option<String>,
}