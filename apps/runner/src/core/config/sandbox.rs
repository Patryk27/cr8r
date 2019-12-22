use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type")]
pub enum SandboxConfig {
    Lxd {
        #[serde(rename = "container-name")]
        container_name: String,
    },

    Shell {
        root: String,
    },
}