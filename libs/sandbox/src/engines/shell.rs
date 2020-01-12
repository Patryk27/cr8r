use std::path::{Path, PathBuf};

use anyhow::Result;

use async_trait::async_trait;

use crate::{SandboxEngine, SandboxListener};

mod cmds;

pub struct ShellEngine {
    root: PathBuf,
    listener: SandboxListener,
}

impl ShellEngine {
    pub async fn create(root: PathBuf) -> Result<Self> {
        Ok(Self {
            root,
            listener: SandboxListener::default(),
        })
    }
}

#[async_trait]
impl SandboxEngine for ShellEngine {
    async fn init(&mut self, listener: SandboxListener) -> Result<()> {
        cmds::init(self, listener)
            .await
    }

    async fn destroy(&mut self) -> Result<()> {
        cmds::destroy(self)
            .await
    }

    async fn exec(&mut self, cmd: &str) -> Result<()> {
        cmds::exec(self, cmd)
            .await
    }

    async fn fs_read(&mut self, path: &Path) -> Result<String> {
        cmds::fs_read(self, path)
            .await
    }

    async fn fs_write(&mut self, path: &Path, content: String) -> Result<()> {
        cmds::fs_write(self, path, content)
            .await
    }
}