use lib_sandbox_lxd::LxdContainerConfig;

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

    pub async fn get_env(&mut self, key: &str) -> Result<String> {
        // @todo `key` should be ^[a-zA-Z0-9_]*$

        let value = self.lxd
            .exec(&self.container, &["bash", "-c", &format!("echo ${}", key)])?
            .output().await?;

        Ok(value)
    }
}