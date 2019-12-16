use std::path::PathBuf;

use async_trait::async_trait;

use crate::{Result, SandboxEngine, SandboxListener, SandboxMount};

pub use self::error::ShellEngineError;

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
        unimplemented!()
    }

    async fn destroy(&mut self) -> Result<()> {
        unimplemented!()
    }

    async fn exec(&mut self, cmd: &str) -> Result<()> {
        unimplemented!()
    }

    async fn mount(&mut self, mount: SandboxMount) -> Result<()> {
        unimplemented!()
    }
}