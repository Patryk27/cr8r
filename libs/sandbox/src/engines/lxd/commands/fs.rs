use std::path::Path;

use anyhow::*;
use log::*;
use tokio::fs;

use lib_core_tempfile::TempFile;

use crate::engines::LxdSandboxEngine;

pub async fn fs_read(engine: &mut LxdSandboxEngine, path: &Path) -> Result<String> {
    trace!("Executing: fs_read(path=`{}`)", path.display());

    let path = engine.config.root.join(path);

    trace!(".. actual path: {}", path.display());

    let proxy_file = TempFile::new().await?;
    let proxy_path = proxy_file.path();

    trace!(".. using proxy file: {}", proxy_path.display());

    engine.client.file_pull(
        &engine.config.container,
        path,
        proxy_path,
    ).await?;

    let content = fs::read_to_string(proxy_path).await?;

    trace!(".. ok, {} bytes read", content.len());

    Ok(content)
}

pub async fn fs_write(engine: &mut LxdSandboxEngine, path: &Path, content: String) -> Result<()> {
    trace!("Executing: fs_write(path=`{}`, content=`{} bytes`", path.display(), content.len());

    let path = engine.config.root.join(path);

    trace!(".. actual path: {}", path.display());

    let proxy_file = TempFile::new().await?;
    let proxy_path = proxy_file.path();

    trace!(".. using proxy file: {}", proxy_path.display());

    fs::write(proxy_path, content).await?;

    engine.client.file_push(
        &engine.config.container,
        proxy_path,
        path,
    ).await?;

    trace!(".. ok");

    Ok(())
}
