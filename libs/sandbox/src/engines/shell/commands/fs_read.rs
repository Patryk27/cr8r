use std::fs;
use std::path::Path;

use anyhow::Result;
use log::*;

use crate::engines::ShellEngine;

pub async fn fs_read(engine: &mut ShellEngine, path: &Path) -> Result<String> {
    debug!("fs_read :: path={}", path.display());

    let path = engine.root.join(path);
    let content = fs::read_to_string(path)?;

    debug!("... = {} bytes", content.len());

    Ok(content)
}