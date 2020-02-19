use anyhow::*;

use crate::{Sandbox, SandboxConfig};
use crate::engines::*;

pub struct SandboxProvider {
    config: SandboxConfig,
}

impl SandboxProvider {
    pub async fn new(config: SandboxConfig) -> Result<Self> {
        use SandboxConfig::*;

        match &config {
            Lxd(config) => {
                LxdSandboxEngine::validate(config).await?;
            }

            Shell(config) => {
                ShellSandboxEngine::validate(config).await?;
            }
        }

        Ok(Self { config })
    }

    pub async fn create(&self) -> Result<Sandbox> {
        use SandboxConfig::*;

        let engine = match self.config.clone() {
            Lxd(config) => {
                box LxdSandboxEngine::create(config).await? as _
            }

            Shell(definition) => {
                box ShellSandboxEngine::create(definition).await? as _
            }
        };

        Ok(Sandbox::new(engine))
    }
}