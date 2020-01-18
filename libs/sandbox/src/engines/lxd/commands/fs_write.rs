use std::path::Path;

use anyhow::Result;
use log::*;

use crate::engines::LxdEngine;

pub async fn fs_write(engine: &mut LxdEngine, path: &Path, content: String) -> Result<()> {
    debug!("fs_write :: path={}, content={} bytes", path.display(), content.len());

    unimplemented!()
}
