use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::ShellEngine;

pub async fn fs_read(engine: &mut ShellEngine, path: &Path) -> Result<String> {
    let path = engine.root.join(path);
    let content = fs::read_to_string(path)?;

    Ok(content)
}