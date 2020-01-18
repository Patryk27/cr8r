use std::path::Path;

use anyhow::Result;
use log::*;

use crate::engines::LxdEngine;

pub async fn fs_read(engine: &mut LxdEngine, path: &Path) -> Result<String> {
    debug!("fs_read :: path={}", path.display());

    unimplemented!()
}