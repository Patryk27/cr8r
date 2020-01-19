use std::path::Path;

use anyhow::Result;
use log::*;
use tokio::fs;

use crate::engines::ShellSandboxEngine;

pub async fn fs_write(engine: &mut ShellSandboxEngine, path: &Path, content: String) -> Result<()> {
    debug!("Executing: fs_write(path=`{}`, content=`{} bytes`)", path.display(), content.len());

    let path = engine.config.root.join(path);

    debug!(".. actual path: {}", path.display());

    fs::write(path, content)
        .await?;

    debug!(".. ok");

    Ok(())
}