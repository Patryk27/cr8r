use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Environment {
    #[serde(rename = "default-toolchain", default = "default_toolchain")]
    pub default_toolchain: String,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            default_toolchain: default_toolchain(),
        }
    }
}

fn default_toolchain() -> String {
    "nightly".to_string()
}