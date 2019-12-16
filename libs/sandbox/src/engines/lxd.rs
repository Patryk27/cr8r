use async_trait::async_trait;
use lib_lxd::{LxdClient, LxdContainerName, LxdImageName};

use crate::{Result, SandboxEngine, SandboxListener, SandboxMount};

pub use self::error::*;

mod cmds;
mod error;

pub struct LxdEngine {
    client: LxdClient,
    container: LxdContainerName,
    listener: SandboxListener,
    mount_idx: usize,
}

impl LxdEngine {
    pub async fn create(container: LxdContainerName, image: LxdImageName) -> Result<Self> {
        let client = LxdClient::autodetect()?;

        Ok(Self {
            client,
            container,
            listener: SandboxListener::default(),
            mount_idx: 0,
        })
    }

    pub fn add_env(&mut self, key: &str, value: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn get_env(&mut self, key: &str) -> Result<String> {
        unimplemented!()
    }

    fn invoke(&mut self, ) {

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

    async fn mount(&mut self, mount: SandboxMount) -> Result<()> {
        cmds::mount(self, mount)
            .await
    }
}