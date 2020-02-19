#![feature(box_syntax)]
#![feature(crate_visibility_modifier)]
#![feature(try_blocks)]
#![feature(type_ascription)]

use std::path::Path;

use anyhow::*;

pub use self::{
    config::*,
    engine::*,
    listener::*,
    provider::*,
};

mod config;
mod engine;
mod engines;
mod listener;
mod provider;

pub struct Sandbox {
    engine: Box<dyn SandboxEngine>,
}

impl Sandbox {
    pub fn new(engine: Box<dyn SandboxEngine>) -> Self {
        Self { engine }
    }

    pub async fn init(&mut self, listener: SandboxListener) -> Result<()> {
        self.engine.init(listener).await
    }

    pub async fn destroy(&mut self) -> Result<()> {
        self.engine.destroy().await
    }

    pub async fn exec(&mut self, cmd: &str) -> Result<()> {
        self.engine.exec(cmd).await
    }

    pub async fn fs_read(&mut self, path: impl AsRef<Path>) -> Result<String> {
        self.engine.fs_read(path.as_ref()).await
    }

    pub async fn fs_write(&mut self, path: impl AsRef<Path>, content: String) -> Result<()> {
        self.engine.fs_write(path.as_ref(), content).await
    }
}
