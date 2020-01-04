use anyhow::{Context, Result};
use tokio::fs;

use crate::{SandboxListener, ShellEngine};

pub async fn init(engine: &mut ShellEngine, listener: SandboxListener) -> Result<()> {
    engine.listener = listener;

    (try {
        create_root_dir_if_not_exists(engine)
            .await?;

        ensure_root_dir_is_writable(engine)
            .await?;

        clean_root_dir(engine)
            .await?;
    }: Result<_>).context("Could not prepare root directory")?;

    Ok(())
}

async fn create_root_dir_if_not_exists(engine: &ShellEngine) -> Result<()> {
    if !engine.root.exists() {
        fs::create_dir(&engine.root)
            .await?;
    }

    Ok(())
}

async fn ensure_root_dir_is_writable(engine: &ShellEngine) -> Result<()> {
    let file = engine.root.join(".test");

    fs::write(&file, "Hello World!")
        .await?;

    fs::remove_file(&file)
        .await?;

    Ok(())
}

async fn clean_root_dir(engine: &ShellEngine) -> Result<()> {
    let mut entries = fs::read_dir(&engine.root).await?;

    while let Some(entry) = entries.next_entry().await? {
        if entry.metadata().await?.is_dir() {
            fs::remove_dir_all(entry.path())
                .await?;
        } else {
            fs::remove_file(entry.path())
                .await?;
        }
    }

    Ok(())
}