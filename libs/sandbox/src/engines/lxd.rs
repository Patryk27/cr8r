use std::path::Path;

use anyhow::Result;

use async_trait::async_trait;
use lib_lxd::{LxdClient, LxdContainerName, LxdImageName};

use crate::{SandboxEngine, SandboxListener};

mod cmds;

pub struct LxdEngine {
    lxd: LxdClient,
    container: LxdContainerName,
    image: LxdImageName,
    listener: SandboxListener,
}

impl LxdEngine {
    pub async fn create(container: LxdContainerName, image: LxdImageName) -> Result<Self> {
        Ok(Self {
            lxd: LxdClient::autodetect().await?,
            container,
            image,
            listener: SandboxListener::default(),
        })
    }
}

#[async_trait]
impl SandboxEngine for LxdEngine {
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
        unimplemented!()
    }

    async fn fs_write(&mut self, path: &Path, content: String) -> Result<()> {
        unimplemented!()
    }
}