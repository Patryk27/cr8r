use std::path::Path;

use anyhow::Result;

use crate::ShellEngine;

pub async fn fs_write(engine: &mut ShellEngine, path: &Path, content: String) -> Result<()> {
    unimplemented!()
}