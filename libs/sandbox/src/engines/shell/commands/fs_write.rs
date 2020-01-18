use std::fs;
use std::path::Path;

use anyhow::Result;
use log::*;

use crate::engines::ShellEngine;

pub async fn fs_write(engine: &mut ShellEngine, path: &Path, content: String) -> Result<()> {
    debug!("fs_write :: path={}, content={} bytes", path.display(), content.len());

    let path = engine.root.join(path);

    fs::write(path, content)?;

    Ok(())
}