//! This module contains an asynchronous wrapper over `tempfile::NamedTempFile`.

#![feature(async_closure)]

use std::fs::File as StdFile;
use std::path::{Path, PathBuf};

use anyhow::*;
use tempfile::NamedTempFile;
use tokio::fs::File as TokioFile;
use tokio::task;

pub struct TempFile {
    inner: Option<NamedTempFile>,
}

impl TempFile {
    pub async fn new() -> Result<Self> {
        let inner = task::spawn_blocking(NamedTempFile::new).await?;

        Ok(Self {
            inner: Some(inner?),
        })
    }

    pub fn path(&self) -> &Path {
        self.inner
            .as_ref()
            .unwrap()
            .path()
    }

    pub fn path_buf(&self) -> PathBuf {
        self.path()
            .to_owned()
    }

    pub fn std_file(&self) -> &StdFile {
        self.inner
            .as_ref()
            .unwrap()
            .as_file()
    }

    pub async fn tokio_file(&self) -> Result<TokioFile> {
        TokioFile::open(self.path()).await
            .map_err(Into::into)
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            // `tempfile` contains a custom destructor that calls `fs::remove_file()` - since it's a blocking call, it
            // has to be handled by a dedicated thread pool

            task::spawn_blocking(async move || {
                drop(inner);
            });
        }
    }
}