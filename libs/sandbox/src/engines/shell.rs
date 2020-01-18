use std::path::{Path, PathBuf};

use anyhow::Result;

use async_trait::async_trait;

use crate::{SandboxEngine, SandboxListener};

pub use self::{
    config::*,
    error::*,
};

mod commands;
mod config;
mod error;

pub struct ShellEngine {
    root: PathBuf,
    listener: SandboxListener,
}

impl ShellEngine {
    pub async fn create(ShellConfig { root }: ShellConfig) -> Result<Self> {
        Ok(Self {
            root,
            listener: SandboxListener::default(),
        })
    }
}

#[async_trait]
impl SandboxEngine for ShellEngine {
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
        commands::fs_read(self, path)
            .await
    }

    async fn fs_write(&mut self, path: &Path, content: String) -> Result<()> {
        commands::fs_write(self, path, content)
            .await
    }
}