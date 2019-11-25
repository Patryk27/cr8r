use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Environment {
    #[serde(rename = "default-os")]
    pub default_os: String,

    #[serde(rename = "default-toolchain")]
    pub default_toolchain: String,
}