use std::path::Path;

use anyhow::*;

use async_trait::async_trait;
use lib_lxd::LxdClient;

use crate::{SandboxEngine, SandboxListener};

pub use self::config::*;

mod commands;
mod config;

pub struct LxdSandboxEngine {
    config: LxdSandboxConfig,
    client: LxdClient,
    listener: SandboxListener,
}

impl LxdSandboxEngine {
    pub async fn validate(_: &LxdSandboxConfig) -> Result<()> {
        LxdClient::autodetect()
            .await?;

        Ok(())
    }

    pub async fn create(config: LxdSandboxConfig) -> Result<Self> {
        Ok(Self {
            config,
            client: LxdClient::autodetect().await?,
            listener: SandboxListener::default(),
        })
    }
}

#[async_trait]
impl SandboxEngine for LxdSandboxEngine {
    async fn init(&mut self, listener: SandboxListener) -> Result<()> {
        commands::init(self, listener)
            .await
    }

    async fn destroy(&mut self) -> Result<()> {
        commands::destroy(self)
            .await
    }

    async fn exec(&mut self, cmd: &str) -> Result<()> {
        commands::exec(self, cmd)
            .await
    }

    async fn fs_read(&mut self, path: &Path) -> Result<String> {
        commands::fs_read(self, path.as_ref())
            .await
    }

    async fn fs_write(&mut self, path: &Path, content: String) -> Result<()> {
        commands::fs_write(self, path, content)
            .await
    }
}