use std::path::PathBuf;

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
}