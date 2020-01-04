use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Controller {
    pub listen: String,

    #[serde(default)]
    pub secret: Option<String>,
}