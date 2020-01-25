use std::path::Path;

use anyhow::*;
use log::*;
use tokio::fs;

use crate::engines::ShellSandboxEngine;

pub async fn fs_write(engine: &mut ShellSandboxEngine, path: &Path, content: String) -> Result<()> {
    trace!("Executing: fs_write(path=`{}`, content=`{} bytes`)", path.display(), content.len());

    let path = engine.config.root.join(path);

    trace!(".. actual path: {}", path.display());

    fs::write(path, content)
        .await?;

    trace!(".. ok");

    Ok(())
}