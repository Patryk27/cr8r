use serde::Deserialize;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Environment {
    #[serde(default = "default_toolchain")]
    crate default_toolchain: String,
}

#[cfg(test)]
impl Default for Environment {
    fn default() -> Self {
        Self {
            default_toolchain: "nightly".into(),
        }
    }
}

fn default_toolchain() -> String {
    "nightly".to_string()
}