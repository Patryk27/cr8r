use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct EcosystemEnvironmentConfig {
    #[serde(default = "default_toolchain")]
    pub default_toolchain: String,
}

impl Default for EcosystemEnvironmentConfig {
    fn default() -> Self {
        Self {
            default_toolchain: default_toolchain(),
        }
    }
}

fn default_toolchain() -> String {
    "nightly".to_string()
}