use std::path::Path;

use anyhow::*;
use tokio::fs::metadata;

use async_trait::async_trait;

use crate::{SandboxEngine, SandboxListener};

pub use self::config::*;

mod commands;
mod config;

pub struct ShellSandboxEngine {
    config: ShellSandboxConfig,
    listener: SandboxListener,
}

impl ShellSandboxEngine {
    pub async fn validate(config: &ShellSandboxConfig) -> Result<()> {
        let meta = metadata(&config.root).await
            .with_context(|| format!("Could not access root directory: {}", config.root.display()))?;

        ensure!(
            meta.is_dir(),
            "Root directory is not actually a directory: {}", config.root.display(),
        );

        Ok(())
    }

    pub async fn create(config: ShellSandboxConfig) -> Result<Self> {
        Ok(Self {
            config,
            listener: SandboxListener::default(),
        })
    }
}

#[async_trait]
impl SandboxEngine for ShellSandboxEngine {
    async fn init(&mut self, listener: SandboxListener) -> Result<()> {
        commands::init(self, listener).await
    }

    async fn destroy(&mut self) -> Result<()> {
        commands::destroy(self).await
    }

    async fn exec(&mut self, cmd: &str) -> Result<()> {
        commands::exec(self, cmd).await
    }

    async fn fs_read(&mut self, path: &Path) -> Result<String> {
        commands::fs_read(self, path).await
    }

    async fn fs_write(&mut self, path: &Path, content: String) -> Result<()> {
        commands::fs_write(self, path, content).await
    }
}