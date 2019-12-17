use std::path::PathBuf;

use async_trait::async_trait;

use crate::{Result, SandboxEngine, SandboxListener};

// We're exporting only the main struct to avoid exporting snafu-related types
pub use self::error::ShellEngineError;

mod cmds;
mod error;

pub struct ShellEngine {
    dir: PathBuf,
    listener: SandboxListener,
}

impl ShellEngine {
    pub async fn create(dir: PathBuf) -> Result<Self> {
        Ok(Self {
            dir,
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