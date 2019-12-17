use async_trait::async_trait;

use crate::{Result, SandboxListener};

#[async_trait]
pub trait SandboxEngine: Send {
    async fn init(&mut self, listener: SandboxListener) -> Result<()>;
    async fn destroy(&mut self) -> Result<()>;
    async fn exec(&mut self, cmd: &str) -> Result<()>;
}