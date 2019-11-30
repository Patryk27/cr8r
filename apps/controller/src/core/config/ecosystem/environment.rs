use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Environment {
    #[serde(rename = "default-system")]
    pub default_system: String,

    #[serde(rename = "default-toolchain")]
    pub default_toolchain: String,
}