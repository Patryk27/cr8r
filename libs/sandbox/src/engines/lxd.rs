use std::path::Path;

use anyhow::Result;

use async_trait::async_trait;
use lib_lxd::{LxdClient, LxdContainerName, LxdImageName};

use crate::{SandboxEngine, SandboxListener};

pub use self::{
    config::*,
    error::*,
};

mod commands;
mod config;
mod error;

pub struct LxdEngine {
    client: LxdClient,
    container: LxdContainerName,
    image: LxdImageName,
    listener: SandboxListener,
}

impl LxdEngine {
    pub async fn create(LxdConfig { container, image }: LxdConfig) -> Result<Self> {
        Ok(Self {
            client: LxdClient::autodetect().await?,
            container,
            image,
            listener: SandboxListener::default(),
        })
    }
}

#[async_trait]
impl SandboxEngine for LxdEngine {
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