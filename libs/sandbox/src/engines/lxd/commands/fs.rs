use std::path::Path;
use std::thread;

use anyhow::Result;
use log::*;
use tempfile::NamedTempFile;
use tokio::{fs, task};

use crate::engines::LxdSandboxEngine;

pub async fn fs_read(engine: &mut LxdSandboxEngine, path: &Path) -> Result<String> {
    debug!("Executing: fs_read(path=`{}`)", path.display());

    let path = engine.config.root.join(path);

    debug!(".. actual path: {}", path.display());

    let proxy_file = create_proxy_file()
        .await?;

    let proxy_path = proxy_file.path();

    debug!(".. using proxy file: {}", proxy_path.display());

    engine
        .client
        .file_pull(&engine.config.container, path, &proxy_file)
        .await?;

    let content = fs::read_to_string(&proxy_file)
        .await?;

    // `NamedTempFile` has a custom `Drop` implementation that's blocking, so it must be sent to some other thread not
    // to jam up our executor
    thread::spawn(move || {
        drop(proxy_file);
    });

    debug!(".. ok, {} bytes read", content.len());

    Ok(content)
}

pub async fn fs_write(engine: &mut LxdSandboxEngine, path: &Path, content: String) -> Result<()> {
    debug!("Executing: fs_write(path=`{}`, content=`{} bytes`", path.display(), content.len());

    let path = engine.config.root.join(path);

    debug!(".. actual path: {}", path.display());

    let proxy_file = create_proxy_file()
        .await?;

    let proxy_path = proxy_file.path();

    debug!(".. using proxy file: {}", proxy_path.display());

    fs::write(proxy_path, content)
        .await?;

    engine
        .client
        .file_push(&engine.config.container, path, &proxy_file)
        .await?;

    // `NamedTempFile` has a custom `Drop` implementation that's blocking, so it must be sent to some other thread not
    // to jam up our executor
    thread::spawn(move || {
        drop(proxy_file);
    });

    debug!(".. ok");

    Ok(())
}

async fn create_proxy_file() -> Result<NamedTempFile> {
    // Since the `tempfile` crate doesn't provide an async version of `NamedTempFile::new()`, we've gotta spawn it on
    // the thread-pool

    let file = task::spawn_blocking(NamedTempFile::new)
        .await?;

    Ok(file?)
}