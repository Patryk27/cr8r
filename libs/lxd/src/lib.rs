#![feature(crate_visibility_modifier)]

use std::path::{Path, PathBuf};

use anyhow::*;

pub use self::{
    listener::*,
    models::*,
};
use self::connector::*;

mod commands;
mod connector;
mod listener;
mod models;

pub struct LxdClient {
    conn: LxdConnector,
}

impl LxdClient {
    pub fn new(path: PathBuf) -> Self {
        Self {
            conn: LxdConnector::new(path),
        }
    }

    pub async fn autodetect() -> Result<Self> {
        LxdConnector::autodetect()
            .await
            .map(|conn| Self { conn })
    }

    pub fn set_listener(&mut self, listener: LxdListener) {
        self.conn.set_listener(listener);
    }

    pub async fn config_device_add(
        &self,
        container: &LxdContainerName,
        dev_name: LxdDeviceName,
        dev_def: LxdDeviceDef,
    ) -> Result<()> {
        commands::config_device_add(&self.conn, container, dev_name, dev_def)
            .await
    }

    pub async fn config_set(
        &self,
        container: &LxdContainerName,
        cfg_key: String,
        cfg_value: String,
    ) -> Result<()> {
        commands::config_set(&self.conn, container, cfg_key, cfg_value)
            .await
    }

    pub async fn delete(&self, container: &LxdContainerName) -> Result<()> {
        commands::delete(&self.conn, container)
            .await
    }

    pub async fn exec(&self, container: &LxdContainerName, args: &[&str]) -> Result<String> {
        commands::exec(&self.conn, container, args)
            .await
    }

    pub async fn file_pull(
        &self,
        container: &LxdContainerName,
        container_file: impl AsRef<Path>,
        host_file: impl AsRef<Path>,
    ) -> Result<()> {
        commands::file_pull(&self.conn, container, container_file.as_ref(), host_file.as_ref())
            .await
    }

    pub async fn file_push(
        &self,
        container: &LxdContainerName,
        host_file: impl AsRef<Path>,
        container_file: impl AsRef<Path>,
    ) -> Result<()> {
        commands::file_push(&self.conn, container, host_file.as_ref(), container_file.as_ref())
            .await
    }

    pub async fn launch(&self, image: &LxdImageName, container: &LxdContainerName) -> Result<()> {
        commands::launch(&self.conn, image, container)
            .await
    }

    pub async fn list(&self) -> Result<Vec<LxdContainer>> {
        commands::list(&self.conn)
            .await
    }
}
