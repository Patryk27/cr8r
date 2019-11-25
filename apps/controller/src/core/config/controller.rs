use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Controller {
    pub bind: String,

    #[serde(rename = "client-secret")]
    pub client_secret: String,

    #[serde(rename = "runner-secret")]
    pub runner_secret: String,
}