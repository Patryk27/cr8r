use lib_lxd::LxdContainerConfig;

use crate::{Result, Sandbox};

impl Sandbox {
    pub async fn add_env(&mut self, key: &str, value: String) -> Result<()> {
        self.invoke(|lxd| {
            lxd.config(&self.container, LxdContainerConfig::Set {
                key: format!("environment.{}", key),
                value,
            })
        }).await
    }
}