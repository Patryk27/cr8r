use std::path::Path;

use anyhow::Result;
use log::*;
use tokio::fs;

use crate::engines::ShellSandboxEngine;

pub async fn fs_read(engine: &mut ShellSandboxEngine, path: &Path) -> Result<String> {
    debug!("Executing: fs_read(path=`{}`)", path.display());

    let path = engine.config.root.join(path);

    debug!(".. actual path = {}", path.display());

    let content = fs::read_to_string(path)
        .await?;

    debug!(".. ok, {} bytes read", content.len());

    Ok(content)
}