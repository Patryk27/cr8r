use std::fs::File;
use std::path::{Path, PathBuf};
use std::thread;

use anyhow::*;
use tempfile::NamedTempFile;
use tokio::task;

pub struct TempFile {
    inner: Option<NamedTempFile>,
}

impl TempFile {
    pub async fn new() -> Result<Self> {
        let inner = task::spawn_blocking(NamedTempFile::new)
            .await?;

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

    pub fn file(&self) -> &File {
        self.inner
            .as_ref()
            .unwrap()
            .as_file()
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            thread::spawn(move || {
                drop(inner);
            });
        }
    }
}