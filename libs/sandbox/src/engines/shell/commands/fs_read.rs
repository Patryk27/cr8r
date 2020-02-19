use std::path::Path;

use anyhow::*;
use log::*;
use tokio::fs;

use crate::engines::ShellSandboxEngine;

pub async fn fs_read(engine: &mut ShellSandboxEngine, path: &Path) -> Result<String> {
    trace!("Executing: fs_read(path=`{}`)", path.display());

    let path = engine.config.root.join(path);

    trace!(".. actual path = {}", path.display());

    let content = fs::read_to_string(path).await?;

    trace!(".. ok, {} bytes read", content.len());

    Ok(content)
}