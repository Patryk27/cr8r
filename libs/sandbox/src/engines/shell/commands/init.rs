use anyhow::*;
use log::*;
use tokio::fs;

use crate::engines::ShellSandboxEngine;
use crate::SandboxListener;

pub async fn init(engine: &mut ShellSandboxEngine, listener: SandboxListener) -> Result<()> {
    trace!("Executing: init()");

    engine.listener = listener;

    (try {
        ensure_root_dir_is_writable(engine).await?;
        clean_root_dir(engine).await?;
    }: Result<()>).context("Could not prepare root directory")?;

    Ok(())
}

async fn ensure_root_dir_is_writable(engine: &ShellSandboxEngine) -> Result<()> {
    trace!(".. ensuring root directory is writable");

    let file = engine.config.root.join(".test");

    fs::write(&file, "Hello World!").await?;
    fs::remove_file(&file).await?;

    trace!(".. .. ok");

    Ok(())
}

async fn clean_root_dir(engine: &ShellSandboxEngine) -> Result<()> {
    trace!(".. cleaning root directory");

    let mut entries = fs::read_dir(&engine.config.root).await?;

    while let Some(entry) = entries.next_entry().await? {
        let entry_meta = entry.metadata().await?;
        let entry_path = entry.path();

        if entry_meta.is_dir() {
            trace!(".. .. removing directory: {}", entry_path.display());
            fs::remove_dir_all(entry_path).await?;
        } else {
            trace!(".. .. removing file: {}", entry_path.display());
            fs::remove_file(entry_path).await?;
        }
    }

    Ok(())
}