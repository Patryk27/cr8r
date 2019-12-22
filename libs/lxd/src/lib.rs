#![feature(box_syntax)]
#![feature(crate_visibility_modifier)]

use std::path::{Path, PathBuf};

pub use self::{
    error::*,
    listener::*,
    models::*,
};

mod cmds;
mod error;
mod listener;
mod models;

pub struct LxdClient {
    path: PathBuf,
    listener: LxdListener,
}

impl LxdClient {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.into(),
            listener: LxdListener::default(),
        }
    }

    pub async fn autodetect() -> Result<Self> {
        cmds::autodetect()
            .await
    }

    pub fn set_listener(&mut self, listener: LxdListener) {
        self.listener = listener;
    }

    pub async fn config(&self, container: &LxdContainerName, config: LxdContainerConfig) -> Result<()> {
        cmds::config(self, container, config)
            .await
    }

    pub async fn delete(&self, container: &LxdContainerName) -> Result<()> {
        cmds::delete(self, container)
            .await
    }

    pub async fn exec(&self, container: &LxdContainerName, args: &[&str]) -> Result<String> {
        cmds::exec(self, container, args)
            .await
    }

    pub async fn launch(&self, image: &LxdImageName, container: &LxdContainerName) -> Result<()> {
        cmds::launch(self, image, container)
            .await
    }

    pub async fn list(&self) -> Result<Vec<LxdContainer>> {
        cmds::list(self)
            .await
    }
}