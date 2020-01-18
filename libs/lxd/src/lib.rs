#![feature(box_syntax)]
#![feature(crate_visibility_modifier)]

use std::path::{Path, PathBuf};

use anyhow::Result;

pub use self::{
    listener::*,
    models::*,
};

mod commands;
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
        commands::autodetect()
            .await
    }

    pub fn set_listener(&mut self, listener: LxdListener) {
        self.listener = listener;
    }

    pub async fn config(&self, container: &LxdContainerName, config: LxdContainerConfig) -> Result<()> {
        commands::config(self, container, config)
            .await
    }

    pub async fn delete(&self, container: &LxdContainerName) -> Result<()> {
        commands::delete(self, container)
            .await
    }

    pub async fn exec(&self, container: &LxdContainerName, args: &[&str]) -> Result<String> {
        commands::exec(self, container, args)
            .await
    }

    /// Models the `lxc file pull` command, allowing one to copy a single file from container into the host.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use lib_lxd::LxdClient;
    ///
    /// let lxd = LxdClient::autodetect()
    ///     .await?;
    ///
    /// lxc.file_pull(
    ///   "my-container".parse()?,
    ///   "/home/ubuntu/hello-world.txt",
    ///   "/tmp/hello-world.txt",
    /// ).await?;
    /// ```
    pub async fn file_pull(
        &self,
        container: &LxdContainerName,
        container_file: impl AsRef<Path>,
        host_file: impl AsRef<Path>,
    ) -> Result<()> {
        commands::file_pull(self, container, container_file.as_ref(), host_file.as_ref())
            .await
    }

    /// Models the `lxc file push` command, allowing one to copy a single file from host into the container.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use lib_lxd::LxdClient;
    ///
    /// let lxd = LxdClient::autodetect()
    ///     .await?;
    ///
    /// lxc.file_push(
    ///   "my-container".parse(),
    ///   "/tmp/hello-world.txt",
    ///   "/home/ubuntu/hello-world.txt",
    /// ).await?;
    /// ```
    pub async fn file_push(
        &self,
        container: &LxdContainerName,
        host_file: impl AsRef<Path>,
        container_file: impl AsRef<Path>,
    ) -> Result<()> {
        commands::file_push(self, container, host_file.as_ref(), container_file.as_ref())
            .await
    }

    pub async fn launch(&self, image: &LxdImageName, container: &LxdContainerName) -> Result<()> {
        commands::launch(self, image, container)
            .await
    }

    pub async fn list(&self) -> Result<Vec<LxdContainer>> {
        commands::list(self)
            .await
    }
}
